//! Playwright integration for headless browser automation.
//!
//! This module contains the inline Playwright scripts, error mapping,
//! and availability checks for Node.js and Playwright.

use crate::{DpcError, Result};
use std::io;
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;

/// Basic Playwright script for screenshot capture.
pub(crate) const PLAYWRIGHT_SCRIPT: &str = r#"
const [, url, width, height, navTimeout, idleTimeout, screenshotPath, headlessFlag] = process.argv;

async function run() {
  let browser;
  try {
    const { chromium } = require('playwright');
    browser = await chromium.launch({ headless: headlessFlag !== '0' });
    const context = await browser.newContext({
      viewport: {
        width: parseInt(width, 10),
        height: parseInt(height, 10)
      }
    });
    const page = await context.newPage();
    const navMs = parseInt(navTimeout, 10);
    const idleMs = parseInt(idleTimeout, 10);

    await page.goto(url, { waitUntil: 'networkidle', timeout: navMs });
    await page.waitForLoadState('networkidle', { timeout: idleMs });

    if (screenshotPath) {
      await page.screenshot({ path: screenshotPath, fullPage: true });
    }

    console.log(JSON.stringify({ status: 'ok' }));
  } catch (err) {
    const message = err && err.message ? err.message : String(err);
    console.error(JSON.stringify({ status: 'error', message }));
    process.exitCode = 1;
  } finally {
    if (browser) {
      await browser.close();
    }
  }
}

run();
"#;

/// Playwright script that captures both screenshot and DOM snapshot.
pub(crate) const PLAYWRIGHT_SCRIPT_WITH_DOM: &str = r#"
const [, url, width, height, navTimeout, idleTimeout, screenshotPath, headlessFlag] = process.argv;

async function run() {
  let browser;
  try {
    const { chromium } = require('playwright');
    browser = await chromium.launch({ headless: headlessFlag !== '0' });
    const context = await browser.newContext({
      viewport: {
        width: parseInt(width, 10),
        height: parseInt(height, 10)
      }
    });
    const page = await context.newPage();
    const navMs = parseInt(navTimeout, 10);
    const idleMs = parseInt(idleTimeout, 10);

    await page.goto(url, { waitUntil: 'networkidle', timeout: navMs });
    await page.waitForLoadState('networkidle', { timeout: idleMs });

    if (screenshotPath) {
      await page.screenshot({ path: screenshotPath, fullPage: false });
    }

    // Extract DOM snapshot
    const domSnapshot = await page.evaluate(() => {
      const nodes = [];
      let nodeId = 0;
      const nodeMap = new Map();

      function getComputedStyleInfo(el) {
        const style = window.getComputedStyle(el);
        const letterSpacing = parseFloat(style.letterSpacing);
        return {
          fontFamily: style.fontFamily || null,
          fontSize: parseFloat(style.fontSize) || null,
          fontWeight: style.fontWeight || null,
          lineHeight: parseFloat(style.lineHeight) || null,
          letterSpacing: Number.isNaN(letterSpacing) ? null : letterSpacing,
          color: style.color || null,
          backgroundColor: style.backgroundColor || null,
          display: style.display || null,
          visibility: style.visibility || null,
          opacity: style.opacity !== '' ? parseFloat(style.opacity) : null
        };
      }

      function traverse(node, parentId) {
        if (node.nodeType !== Node.ELEMENT_NODE) return null;

        const el = node;
        const id = `node-${nodeId++}`;
        nodeMap.set(el, id);

        const rect = el.getBoundingClientRect();
        const tag = el.tagName.toLowerCase();

        // Skip invisible elements
        if (rect.width === 0 && rect.height === 0) return null;

        const attributes = {};
        for (const attr of el.attributes) {
          attributes[attr.name] = attr.value;
        }

        // Get direct text content (not from children)
        let text = null;
        for (const child of el.childNodes) {
          if (child.nodeType === Node.TEXT_NODE) {
            const trimmed = child.textContent.trim();
            if (trimmed) {
              text = text ? text + ' ' + trimmed : trimmed;
            }
          }
        }

        const childIds = [];
        for (const child of el.children) {
          const childId = traverse(child, id);
          if (childId) childIds.push(childId);
        }

        nodes.push({
          id,
          tag,
          children: childIds,
          parent: parentId,
          attributes,
          text,
          boundingBox: {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height
          },
          computedStyle: getComputedStyleInfo(el)
        });

        return id;
      }

      traverse(document.body, null);

      return {
        url: window.location.href,
        title: document.title,
        nodes
      };
    });

    console.log(JSON.stringify({ status: 'ok', dom: domSnapshot }));
  } catch (err) {
    const message = err && err.message ? err.message : String(err);
    console.error(JSON.stringify({ status: 'error', message }));
    process.exitCode = 1;
  } finally {
    if (browser) {
      await browser.close();
    }
  }
}

run();
"#;

/// Timeout for checking node/playwright availability.
pub(crate) const NODE_CHECK_TIMEOUT: Duration = Duration::from_secs(5);

/// Script to check if Playwright is installed.
const PLAYWRIGHT_CHECK_SCRIPT: &str = "require('playwright'); process.stdout.write('ok');";

/// Simple script result for basic Playwright operations.
#[derive(Debug, serde::Deserialize)]
pub(crate) struct ScriptResult {
    pub status: String,
    pub message: Option<String>,
}

/// Error result from Playwright script.
#[derive(Debug, serde::Deserialize)]
pub(crate) struct ScriptError {
    pub status: String,
    pub message: String,
}

/// Maps a spawn error to an appropriate DpcError.
pub(crate) fn map_spawn_error(err: io::Error, command: &str) -> DpcError {
    if err.kind() == io::ErrorKind::NotFound {
        DpcError::Config(format!(
            "Unable to spawn Playwright helper; '{}' was not found on PATH",
            command
        ))
    } else {
        DpcError::Io(err)
    }
}

/// Maps Playwright stderr output to an appropriate DpcError.
pub(crate) fn map_playwright_error(status_text: impl Into<String>, stderr: &str) -> DpcError {
    if let Ok(error) = serde_json::from_str::<ScriptError>(stderr) {
        return map_playwright_status_error(&error.status, error.message);
    }

    let lower = stderr.to_ascii_lowercase();

    if stderr
        .to_ascii_lowercase()
        .contains("cannot find module 'playwright'")
    {
        return DpcError::Config(
            "Playwright npm package is missing; install with `npm install playwright`.".to_string(),
        );
    }

    if lower.contains("timeout") {
        return DpcError::Config(
            "Playwright timed out; try increasing --nav-timeout/--network-idle-timeout or --process-timeout, and ensure the page finishes loading."
                .to_string(),
        );
    }

    DpcError::Config(format!(
        "Playwright exited with status {}: {}",
        status_text.into(),
        stderr.trim()
    ))
}

/// Maps a Playwright status error to an appropriate DpcError.
pub(crate) fn map_playwright_status_error(status: &str, message: String) -> DpcError {
    if message
        .to_ascii_lowercase()
        .contains("cannot find module 'playwright'")
    {
        DpcError::Config(
            "Playwright npm package is missing; install with `npm install playwright`.".to_string(),
        )
    } else if message.to_ascii_lowercase().contains("timeout") {
        DpcError::Config(format!(
            "Playwright error (status {}): {}. Hint: increase --nav-timeout/--network-idle-timeout or --process-timeout, and ensure the page finishes loading.",
            status, message
        ))
    } else {
        DpcError::Config(format!("Playwright error (status {}): {}", status, message))
    }
}

/// Checks if mock rendering is enabled via environment variables.
pub(crate) fn is_mock_rendering_enabled() -> bool {
    std::env::var("DPC_MOCK_RENDER_REF").is_ok()
        || std::env::var("DPC_MOCK_RENDER_IMPL").is_ok()
        || std::env::var("DPC_MOCK_RENDERERS_DIR").is_ok()
}

/// Ensures Node.js is available on the system.
pub(crate) async fn ensure_node_available(node_command: &str) -> Result<()> {
    let mut cmd = Command::new(node_command);
    cmd.arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    let status = tokio::time::timeout(NODE_CHECK_TIMEOUT, cmd.status())
        .await
        .map_err(|_| {
            DpcError::Config(format!(
                "Timed out checking node availability after {:?}",
                NODE_CHECK_TIMEOUT
            ))
        })?
        .map_err(|err| map_spawn_error(err, node_command))?;

    if !status.success() {
        return Err(DpcError::Config(format!(
            "Node command {:?} is not available (exit {})",
            node_command, status
        )));
    }

    Ok(())
}

/// Ensures Playwright npm package is installed.
pub(crate) async fn ensure_playwright_available(node_command: &str) -> Result<()> {
    if is_mock_rendering_enabled() {
        return Ok(());
    }

    let mut cmd = Command::new(node_command);
    cmd.arg("-e")
        .arg(PLAYWRIGHT_CHECK_SCRIPT)
        .stdout(Stdio::null())
        .stderr(Stdio::piped());

    let output = tokio::time::timeout(NODE_CHECK_TIMEOUT, cmd.output())
        .await
        .map_err(|_| {
            DpcError::Config(format!(
                "Timed out checking Playwright availability after {:?}",
                NODE_CHECK_TIMEOUT
            ))
        })?
        .map_err(|err| map_spawn_error(err, node_command))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(map_playwright_error(
            format!("{:?}", output.status),
            &stderr,
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_playwright_error_detects_missing_module() {
        let err = map_playwright_error(
            "1",
            r#"{"status":"error","message":"Cannot find module 'playwright'"}"#,
        );
        match err {
            DpcError::Config(msg) => {
                assert!(
                    msg.contains("Playwright npm package is missing"),
                    "expected missing playwright hint, got: {msg}"
                );
            }
            other => panic!("expected config error, got {other:?}"),
        }
    }

    #[test]
    fn map_playwright_error_handles_plain_stderr_missing_module() {
        let err = map_playwright_error("1", "Error: Cannot find module 'playwright'");
        match err {
            DpcError::Config(msg) => assert!(
                msg.contains("npm install playwright"),
                "expected npm install hint, got: {msg}"
            ),
            other => panic!("expected config error, got {other:?}"),
        }
    }

    #[test]
    fn script_error_reports_missing_playwright_module() {
        let err = map_playwright_error(
            "exit status: 1",
            r#"{"status":"error","message":"Cannot find module 'playwright'"}"#,
        );
        let msg = format!("{}", err);
        assert!(msg.contains("Playwright npm package is missing"));
    }

    #[test]
    fn map_playwright_error_handles_non_json_missing_module() {
        let err = map_playwright_error(
            "exit status: 1",
            "Error: Cannot find module 'playwright'\n    at Module._resolveFilename",
        );
        let msg = format!("{}", err);
        assert!(
            msg.contains("Playwright npm package is missing"),
            "expected missing playwright hint, got: {msg}"
        );
    }

    #[test]
    fn map_playwright_error_includes_timeout_hint() {
        let err = map_playwright_error(
            "exit status: 1",
            r#"{"status":"error","message":"Navigation timeout of 30000ms exceeded"}"#,
        );
        let msg = format!("{}", err);
        assert!(
            msg.to_ascii_lowercase().contains("timeout"),
            "expected timeout mention, got: {msg}"
        );
        assert!(
            msg.contains("--nav-timeout") || msg.contains("--network-idle-timeout"),
            "expected CLI hint, got: {msg}"
        );
    }

    #[test]
    fn map_playwright_status_error_includes_timeout_hint() {
        let err =
            map_playwright_status_error("1", "Timeout waiting for networkidle state".to_string());
        let msg = format!("{}", err);
        assert!(
            msg.to_ascii_lowercase().contains("timeout"),
            "expected timeout mention, got: {msg}"
        );
        assert!(
            msg.contains("--nav-timeout") || msg.contains("--network-idle-timeout"),
            "expected CLI hint, got: {msg}"
        );
    }

    #[test]
    fn script_error_preserves_other_messages() {
        let err = map_playwright_error(
            "exit status: 1",
            r#"{"status":"error","message":"Timeout navigating to https://example.com"}"#,
        );
        let msg = format!("{}", err);
        assert!(msg.contains("Playwright error"));
        assert!(msg.contains("Timeout navigating"));
    }

    #[tokio::test]
    async fn ensure_node_available_fails_for_missing_binary() {
        let result = ensure_node_available("definitely-not-a-binary").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn ensure_playwright_available_fails_for_missing_binary() {
        let result = ensure_playwright_available("definitely-not-a-binary").await;
        assert!(result.is_err());
    }
}
