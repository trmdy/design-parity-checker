//! Browser automation module for headless page rendering.
//!
//! This module provides functionality for capturing screenshots and DOM snapshots
//! from web pages using Playwright via Node.js.
//!
//! # Module Structure
//!
//! - [`manager`] - Browser session management with concurrency control
//! - [`playwright`] - Playwright scripts and availability checks
//! - [`dom`] - DOM snapshot types and conversion
//!
//! # Example
//!
//! ```no_run
//! use dpc_lib::{BrowserManager, BrowserOptions};
//! use std::path::Path;
//!
//! # async fn example() -> dpc_lib::Result<()> {
//! let manager = BrowserManager::new(BrowserOptions::default());
//! let result = manager.render_url("https://example.com", Some(Path::new("screenshot.png"))).await?;
//! println!("Screenshot saved to {:?}", result.screenshot_path);
//! # Ok(())
//! # }
//! ```

mod dom;
mod manager;
mod playwright;

// Re-export public types from manager
pub use manager::{
    url_to_normalized_view, BrowserManager, BrowserOptions, PageRenderResult, UrlToViewOptions,
    DEFAULT_NAVIGATION_TIMEOUT, DEFAULT_NETWORK_IDLE_TIMEOUT, DEFAULT_PROCESS_TIMEOUT,
};
