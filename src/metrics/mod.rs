//! Metrics module for comparing reference and implementation views.
//!
//! This module provides a unified interface for computing various design parity metrics:
//! - Pixel/perceptual similarity (SSIM-based)
//! - Layout/structure similarity (element matching)
//! - Typography similarity (font properties)
//! - Color palette similarity (k-means clustering)
//! - Content comparison (text matching)

// Submodules
mod color;
mod content;
mod issues;
mod layout;
mod pixel;
mod runner;
mod scoring;
mod typography;
mod hierarchy; // Declare the new hierarchy module

#[cfg(test)]
mod tests;

// Re-exports
pub use crate::types::metric_results::{
    ColorMetric, ColorIssue, LayoutMetric, LayoutIssue, PixelMetric, PixelDiffRegion, DiffSeverity,
    PixelDiffReason, TypographyMetric, TypographyIssue, ContentMetric, ContentIssue,
    HierarchyMetric, HierarchyIssue,
    LayoutDiffKind, LayoutDiffRegion, ColorDiff, ColorDiffKind, TypographyDiff
};

pub use color::ColorPaletteMetric;
pub use content::ContentSimilarity;
pub use issues::{Issue, IssueKind, generate_top_issues};
pub use layout::LayoutSimilarity;
pub use pixel::{cluster_diff_regions, PixelDiffThresholds, PixelSimilarity};
pub use runner::{default_metrics, run_metrics, Metric, MetricKind, MetricResult};
pub use scoring::{calculate_combined_score, ScoreWeights};
pub use typography::TypographySimilarity;
pub use hierarchy::HierarchyHeuristic;