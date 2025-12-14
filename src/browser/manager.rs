//! Browser manager for coordinating headless browser sessions.
//!
//! This module provides the `BrowserManager` struct for managing
//! concurrent browser sessions with semaphore-based limiting.

use crate::types::{NormalizedView, ResourceKind};
use crate::{DpcError, Result, Viewport};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::io::AsyncReadExt;
use tokio::process::Command;
use tokio::sync::Semaphore;
use tokio::time::timeout;

use super::dom::{convert_raw_dom, ScriptResultWithDom};
use super::playwright::{
    ensure_node_available, ensure_playwright_available, map_playwright_error,
    map_playwright_status_error, map_spawn_error, ScriptError, ScriptResult, PLAYWRIGHT_SCRIPT,
    PLAYWRIGHT_SCRIPT_WITH_DOM,
};

/// Default timeout for page navigation.
pub const DEFAULT_NAVIGATION_TIMEOUT: Duration = Duration::from_secs(30);

/// Default timeout for waiting for network idle state.
pub const DEFAULT_NETWORK_IDLE_TIMEOUT: Duration = Duration::from_secs(10);

/// Default timeout for the entire Playwright process.
pub const DEFAULT_PROCESS_TIMEOUT: Duration = Duration::from_secs(45);

/// Configuration options for browser sessions.
#[derive(Debug, Clone)]
pub struct BrowserOptions {
    /// The Node.js command to use (default: "node").
    pub node_command: String,
    /// Viewport dimensions for the browser.
    pub viewport: Viewport,
    /// Whether to run in headless mode.
    pub headless: bool,
    /// Timeout for page navigation.
    pub navigation_timeout: Duration,
    /// Timeout for waiting for network idle state.
    pub network_idle_timeout: Duration,
    /// Timeout for the entire Playwright process.
    pub process_timeout: Duration,
    /// Maximum number of concurrent browser sessions.
    pub max_concurrent_sessions: usize,
}

impl Default for BrowserOptions {
    fn default() -> Self {
        Self {
            node_command: "node".to_string(),
            viewport: Viewport::default(),
            headless: true,
            navigation_timeout: DEFAULT_NAVIGATION_TIMEOUT,
            network_idle_timeout: DEFAULT_NETWORK_IDLE_TIMEOUT,
            process_timeout: DEFAULT_PROCESS_TIMEOUT,
            max_concurrent_sessions: 1,
        }
    }
}

/// Manages concurrent browser sessions with semaphore-based limiting.
#[derive(Debug, Clone)]
pub struct BrowserManager {
    options: BrowserOptions,
    semaphore: Arc<Semaphore>,
}

/// Result of rendering a page.
#[derive(Debug, Clone)]
pub struct PageRenderResult {
    /// Path to the saved screenshot, if any.
    pub screenshot_path: Option<PathBuf>,
    /// Viewport used for rendering.
    pub viewport: Viewport,
    /// Time taken to render the page.
    pub elapsed: Duration,
}

impl BrowserManager {
    /// Creates a new BrowserManager with the given options.
    pub fn new(options: BrowserOptions) -> Self {
        let permits = options.max_concurrent_sessions.max(1);
        Self {
            options,
            semaphore: Arc::new(Semaphore::new(permits)),
        }
    }

    /// Renders a URL and optionally saves a screenshot.
    pub async fn render_url(
        &self,
        url: &str,
        screenshot_path: Option<&Path>,
    ) -> Result<PageRenderResult> {
        self.ensure_node_available().await?;
        let _permit = self
            .semaphore
            .acquire()
            .await
            .map_err(|_| DpcError::Config("Browser manager unavailable".to_string()))?;

        self.run_playwright(url, screenshot_path).await
    }

    /// Render a URL to a full NormalizedView (screenshot + DOM snapshot) using the manager's settings.
    pub async fn render_url_to_normalized_view(
        &self,
        url: &str,
        screenshot_path: &Path,
    ) -> Result<NormalizedView> {
        ensure_node_available(&self.options.node_command).await?;
        let _permit = self
            .semaphore
            .acquire()
            .await
            .map_err(|_| DpcError::Config("Browser manager unavailable".to_string()))?;

        let options: UrlToViewOptions = self.options.clone().into();
        url_to_normalized_view(url, screenshot_path, options).await
    }

    async fn run_playwright(
        &self,
        url: &str,
        screenshot_path: Option<&Path>,
    ) -> Result<PageRenderResult> {
        // Fail fast if Node is missing to avoid spawning Playwright unnecessarily.
        self.ensure_node_available().await?;
        ensure_playwright_available(&self.options.node_command).await?;

        if let Some(path) = screenshot_path {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(|e| {
                    DpcError::Config(format!("Failed to create screenshot dir: {}", e))
                })?;
            }
        }

        let mut cmd = Command::new(&self.options.node_command);
        cmd.arg("-e")
            .arg(PLAYWRIGHT_SCRIPT)
            .arg(url)
            .arg(self.options.viewport.width.to_string())
            .arg(self.options.viewport.height.to_string())
            .arg(self.options.navigation_timeout.as_millis().to_string())
            .arg(self.options.network_idle_timeout.as_millis().to_string())
            .arg(
                screenshot_path
                    .map(|path| path.to_string_lossy().to_string())
                    .unwrap_or_default(),
            )
            .arg(if self.options.headless { "1" } else { "0" })
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let start = Instant::now();
        let mut child = cmd
            .spawn()
            .map_err(|err| map_spawn_error(err, &self.options.node_command))?;

        let stdout_pipe = child.stdout.take();
        let stderr_pipe = child.stderr.take();

        let stdout_task = tokio::spawn(async move {
            let mut buf = Vec::new();
            if let Some(mut out) = stdout_pipe {
                let _ = out.read_to_end(&mut buf).await;
            }
            buf
        });

        let stderr_task = tokio::spawn(async move {
            let mut buf = Vec::new();
            if let Some(mut err) = stderr_pipe {
                let _ = err.read_to_end(&mut buf).await;
            }
            buf
        });

        let status = match timeout(self.options.process_timeout, child.wait()).await {
            Ok(Ok(status)) => status,
            Ok(Err(err)) => return Err(DpcError::Io(err)),
            Err(_) => {
                let _ = child.kill().await;
                let _ = child.wait().await;
                return Err(DpcError::Config(format!(
                    "Playwright timed out after {:?}",
                    self.options.process_timeout
                )));
            }
        };

        let stdout = stdout_task.await.unwrap_or_else(|_| Vec::new());
        let stderr = stderr_task.await.unwrap_or_else(|_| Vec::new());

        if !status.success() {
            let stderr = String::from_utf8_lossy(&stderr);
            return Err(map_playwright_error(status.to_string(), &stderr));
        }

        let stdout = String::from_utf8_lossy(&stdout);
        if let Ok(payload) = serde_json::from_str::<ScriptResult>(&stdout) {
            if payload.status != "ok" {
                let detail = payload
                    .message
                    .as_deref()
                    .unwrap_or("no additional details");
                return Err(DpcError::Config(format!(
                    "Playwright returned non-ok status {}: {}",
                    payload.status, detail
                )));
            }
        } else {
            return Err(DpcError::Config(format!(
                "Unexpected Playwright output: {}",
                stdout.trim()
            )));
        }

        Ok(PageRenderResult {
            screenshot_path: screenshot_path.map(|path| path.to_path_buf()),
            viewport: self.options.viewport,
            elapsed: start.elapsed(),
        })
    }

    async fn ensure_node_available(&self) -> Result<()> {
        ensure_node_available(&self.options.node_command).await
    }
}

/// Options for URL to NormalizedView conversion.
#[derive(Clone)]
pub struct UrlToViewOptions {
    /// The Node.js command to use.
    pub node_command: String,
    /// Viewport dimensions for the browser.
    pub viewport: Viewport,
    /// Whether to run in headless mode.
    pub headless: bool,
    /// Timeout for page navigation.
    pub navigation_timeout: Duration,
    /// Timeout for waiting for network idle state.
    pub network_idle_timeout: Duration,
    /// Timeout for the entire Playwright process.
    pub process_timeout: Duration,
    /// Optional progress callback for logging.
    pub progress: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

impl Default for UrlToViewOptions {
    fn default() -> Self {
        Self {
            node_command: "node".to_string(),
            viewport: Viewport::default(),
            headless: true,
            navigation_timeout: DEFAULT_NAVIGATION_TIMEOUT,
            network_idle_timeout: DEFAULT_NETWORK_IDLE_TIMEOUT,
            process_timeout: DEFAULT_PROCESS_TIMEOUT,
            progress: None,
        }
    }
}

impl From<BrowserOptions> for UrlToViewOptions {
    fn from(opts: BrowserOptions) -> Self {
        Self {
            node_command: opts.node_command,
            viewport: opts.viewport,
            headless: opts.headless,
            navigation_timeout: opts.navigation_timeout,
            network_idle_timeout: opts.network_idle_timeout,
            process_timeout: opts.process_timeout,
            progress: None,
        }
    }
}

fn log_progress(progress: &Option<Arc<dyn Fn(&str) + Send + Sync>>, message: &str) {
    if let Some(cb) = progress {
        cb(message);
    }
}

/// Converts a URL to a NormalizedView by rendering it in a headless browser.
///
/// This function:
/// - Navigates to the URL and waits for network idle
/// - Captures a screenshot at the specified viewport
/// - Extracts the DOM tree with bounding boxes and computed styles
///
/// # Arguments
/// * `url` - The URL to render
/// * `screenshot_path` - Path where the screenshot should be saved
/// * `options` - Configuration options for the browser
///
/// # Returns
/// A NormalizedView containing the screenshot path, viewport dimensions, and DOM snapshot.
pub async fn url_to_normalized_view(
    url: &str,
    screenshot_path: &Path,
    options: UrlToViewOptions,
) -> Result<NormalizedView> {
    let progress = options.progress.clone();
    let nav_secs = options.navigation_timeout.as_secs();
    let idle_secs = options.network_idle_timeout.as_secs();
    log_progress(
        &progress,
        &format!(
            "Launching headless browser for {} ({}x{}, nav {}s, idle {}s)…",
            url, options.viewport.width, options.viewport.height, nav_secs, idle_secs
        ),
    );
    ensure_node_available(&options.node_command).await?;
    ensure_playwright_available(&options.node_command).await?;

    if let Some(parent) = screenshot_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| DpcError::Config(format!("Failed to create screenshot dir: {}", e)))?;
    }

    let mut cmd = Command::new(&options.node_command);
    cmd.arg("-e")
        .arg(PLAYWRIGHT_SCRIPT_WITH_DOM)
        .arg(url)
        .arg(options.viewport.width.to_string())
        .arg(options.viewport.height.to_string())
        .arg(options.navigation_timeout.as_millis().to_string())
        .arg(options.network_idle_timeout.as_millis().to_string())
        .arg(screenshot_path.to_string_lossy().to_string())
        .arg(if options.headless { "1" } else { "0" })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    log_progress(
        &progress,
        "Navigating and waiting for network idle (Playwright)…",
    );
    let start = Instant::now();
    let mut child = cmd
        .spawn()
        .map_err(|err| map_spawn_error(err, &options.node_command))?;

    let stdout_pipe = child.stdout.take();
    let stderr_pipe = child.stderr.take();

    let stdout_task = tokio::spawn(async move {
        let mut buf = Vec::new();
        if let Some(mut out) = stdout_pipe {
            let _ = out.read_to_end(&mut buf).await;
        }
        buf
    });

    let stderr_task = tokio::spawn(async move {
        let mut buf = Vec::new();
        if let Some(mut err) = stderr_pipe {
            let _ = err.read_to_end(&mut buf).await;
        }
        buf
    });

    let status = match timeout(options.process_timeout, child.wait()).await {
        Ok(Ok(status)) => status,
        Ok(Err(err)) => return Err(DpcError::Io(err)),
        Err(_) => {
            let _ = child.kill().await;
            let _ = child.wait().await;
            log_progress(
                &progress,
                "Playwright timed out; process killed after exceeding timeout.",
            );
            return Err(DpcError::Config(format!(
                "Playwright timed out after {:?}",
                options.process_timeout
            )));
        }
    };

    let stdout = stdout_task.await.unwrap_or_else(|_| Vec::new());
    let stderr = stderr_task.await.unwrap_or_else(|_| Vec::new());

    if !status.success() {
        let stderr = String::from_utf8_lossy(&stderr);
        return Err(map_playwright_error(status.to_string(), &stderr));
    }

    let stdout = String::from_utf8_lossy(&stdout);
    let result: ScriptResultWithDom = serde_json::from_str(&stdout).map_err(|e| {
        DpcError::Config(format!(
            "Failed to parse Playwright output: {} - raw: {}",
            e,
            stdout.trim()
        ))
    })?;

    if result.status != "ok" {
        if let Ok(err) = serde_json::from_str::<ScriptError>(&stdout) {
            return Err(map_playwright_status_error(&err.status, err.message));
        }
        return Err(DpcError::Config(format!(
            "Playwright returned non-ok status: {}",
            result.status
        )));
    }

    let dom_data = result.dom.ok_or_else(|| {
        DpcError::Config("Playwright returned ok status but no DOM data".to_string())
    })?;

    let dom_snapshot = convert_raw_dom(dom_data);

    log_progress(
        &progress,
        &format!("Capture finished in {:.1}s", start.elapsed().as_secs_f32()),
    );

    Ok(NormalizedView {
        kind: ResourceKind::Url,
        screenshot_path: screenshot_path.to_path_buf(),
        width: options.viewport.width,
        height: options.viewport.height,
        dom: Some(dom_snapshot),
        figma_tree: None,
        ocr_blocks: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn browser_options_default_values() {
        let opts = BrowserOptions::default();
        assert_eq!(opts.node_command, "node");
        assert!(opts.headless);
        assert_eq!(opts.max_concurrent_sessions, 1);
        assert_eq!(opts.viewport.width, 1440);
        assert_eq!(opts.viewport.height, 900);
        assert_eq!(opts.navigation_timeout, DEFAULT_NAVIGATION_TIMEOUT);
        assert_eq!(opts.network_idle_timeout, DEFAULT_NETWORK_IDLE_TIMEOUT);
        assert_eq!(opts.process_timeout, DEFAULT_PROCESS_TIMEOUT);
    }

    #[test]
    fn semaphore_never_zero() {
        let manager = BrowserManager::new(BrowserOptions {
            max_concurrent_sessions: 0,
            ..BrowserOptions::default()
        });

        assert_eq!(manager.semaphore.available_permits(), 1);
    }

    #[test]
    fn url_to_view_options_default_values() {
        let opts = UrlToViewOptions::default();
        assert_eq!(opts.node_command, "node");
        assert!(opts.headless);
        assert_eq!(opts.viewport.width, 1440);
        assert_eq!(opts.viewport.height, 900);
        assert_eq!(opts.navigation_timeout, DEFAULT_NAVIGATION_TIMEOUT);
        assert_eq!(opts.network_idle_timeout, DEFAULT_NETWORK_IDLE_TIMEOUT);
        assert_eq!(opts.process_timeout, DEFAULT_PROCESS_TIMEOUT);
        assert!(opts.progress.is_none());
    }

    #[test]
    fn url_to_view_options_from_browser_options() {
        let browser_opts = BrowserOptions {
            node_command: "custom-node".to_string(),
            viewport: Viewport {
                width: 1920,
                height: 1080,
            },
            headless: false,
            navigation_timeout: Duration::from_secs(30),
            network_idle_timeout: Duration::from_secs(10),
            process_timeout: Duration::from_secs(60),
            max_concurrent_sessions: 4,
        };

        let view_opts: UrlToViewOptions = browser_opts.into();
        assert_eq!(view_opts.node_command, "custom-node");
        assert!(!view_opts.headless);
        assert_eq!(view_opts.viewport.width, 1920);
        assert_eq!(view_opts.viewport.height, 1080);
        assert_eq!(view_opts.navigation_timeout, Duration::from_secs(30));
        assert_eq!(view_opts.network_idle_timeout, Duration::from_secs(10));
        assert_eq!(view_opts.process_timeout, Duration::from_secs(60));
        assert!(view_opts.progress.is_none());
    }

    #[tokio::test]
    async fn ensure_node_available_fails_for_missing_binary() {
        let manager = BrowserManager::new(BrowserOptions {
            node_command: "definitely-not-a-binary".to_string(),
            ..BrowserOptions::default()
        });

        let result = manager.ensure_node_available().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn render_url_to_normalized_view_checks_node() {
        let manager = BrowserManager::new(BrowserOptions {
            node_command: "definitely-not-a-binary".to_string(),
            ..BrowserOptions::default()
        });

        let result = manager
            .render_url_to_normalized_view("https://example.com", Path::new("tmp.png"))
            .await;

        assert!(result.is_err());
    }
}
