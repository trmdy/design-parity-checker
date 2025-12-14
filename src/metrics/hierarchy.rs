use std::collections::{HashMap, HashSet};

use crate::types::{
    core::{NormalizedView, BoundingBox},
    metric_results::{HierarchyIssue, HierarchyMetric, MetricResult},
    dom::{DomNode, DomSnapshot, ComputedStyle},
    figma::{FigmaNode, FigmaSnapshot},
};

use super::{Metric, MetricKind};

pub struct HierarchyHeuristic {
    // Configuration for hierarchy analysis, e.g., thresholds for tier detection
    pub min_tiers: usize,
    pub max_tiers: usize,
    pub tier_tolerance: f64, // Percentage tolerance for grouping font sizes into tiers
}

impl Default for HierarchyHeuristic {
    fn default() -> Self {
        Self {
            min_tiers: 2, // At least two distinct font sizes for a basic hierarchy
            max_tiers: 5, // No more than 5 distinct font sizes for a clear hierarchy
            tier_tolerance: 0.1, // 10% tolerance for grouping font sizes
        }
    }
}

impl Metric for HierarchyHeuristic {
    fn compute(&self, _ref_view: &NormalizedView, impl_view: &NormalizedView) -> MetricResult {
        let mut font_sizes = Vec::new();
        let mut text_elements = Vec::new();

        if let Some(dom_snapshot) = impl_view.dom.as_ref() {
            self.extract_dom_text_elements(dom_snapshot, &mut font_sizes, &mut text_elements);
        }

        if let Some(figma_snapshot) = impl_view.figma_tree.as_ref() {
            self.extract_figma_text_elements(figma_snapshot, &mut font_sizes, &mut text_elements);
        }

        // Remove duplicates and sort font sizes
        font_sizes.sort_by(|a, b| a.partial_cmp(b).unwrap());
        font_sizes.dedup();

        let (distinct_tiers, tier_count) = self.group_font_sizes_into_tiers(&font_sizes);

        let mut issues = Vec::new();

        if tier_count < self.min_tiers {
            issues.push(HierarchyIssue::TooFewTiers(tier_count));
        } else if tier_count > self.max_tiers {
            issues.push(HierarchyIssue::TooManyTiers(tier_count));
        }

        // Further analysis can be added here to detect "unusual font sizes"
        // by checking if any font size falls outside the established tiers within tolerance.
        // For now, we focus on the tier count.

        let score = self.calculate_score(tier_count);

        MetricResult::Hierarchy(HierarchyMetric {
            score,
            issues,
            distinct_tiers,
            tier_count,
        })
    }

    fn kind(&self) -> MetricKind {
        MetricKind::Hierarchy
    }
}

impl HierarchyHeuristic {
    fn extract_dom_text_elements(&self, dom_snapshot: &DomSnapshot, font_sizes: &mut Vec<f64>, text_elements: &mut Vec<(f64, String, BoundingBox)>) {
        for node in &dom_snapshot.nodes {
            if let Some(text_content) = &node.text {
                if !text_content.trim().is_empty() {
                    if let Some(computed_style) = &node.computed_style {
                        if let Some(font_size) = computed_style.font_size {
                            font_sizes.push(font_size);
                            text_elements.push((font_size, text_content.clone(), node.bounding_box));
                        }
                    }
                }
            }
        }
    }

    fn extract_figma_text_elements(&self, figma_snapshot: &FigmaSnapshot, font_sizes: &mut Vec<f64>, text_elements: &mut Vec<(f64, String, BoundingBox)>) {
        for node in &figma_snapshot.nodes {
            if let Some(typography) = &node.typography {
                if let Some(font_size) = typography.font_size {
                    if let Some(text_content) = &node.text { // Assuming FigmaNode has text_content
                        if !text_content.trim().is_empty() {
                            font_sizes.push(font_size);
                            text_elements.push((font_size, text_content.clone(), node.bounding_box));
                        }
                    }
                }
            }
        }
    }

    fn group_font_sizes_into_tiers(&self, sorted_font_sizes: &[f64]) -> (Vec<f64>, usize) {
        if sorted_font_sizes.is_empty() {
            return (Vec::new(), 0);
        }

        let mut tiers = Vec::new();
        let mut current_tier_representative = sorted_font_sizes[0];
        tiers.push(current_tier_representative);

        for &size in sorted_font_sizes.iter().skip(1) {
            let lower_bound = current_tier_representative * (1.0 - self.tier_tolerance);
            let upper_bound = current_tier_representative * (1.0 + self.tier_tolerance);

            if size < lower_bound || size > upper_bound {
                // This font size is significantly different, start a new tier
                current_tier_representative = size;
                tiers.push(current_tier_representative);
            }
        }

        (tiers, tiers.len())
    }

    fn calculate_score(&self, tier_count: usize) -> f64 {
        // Simple scoring: optimal tier count gets 1.0, deviating reduces score
        if tier_count >= self.min_tiers && tier_count <= self.max_tiers {
            1.0
        } else {
            // Penalize deviations from the optimal range
            let deviation = if tier_count < self.min_tiers {
                (self.min_tiers - tier_count) as f64
            } else {
                (tier_count - self.max_tiers) as f64
            };
            // Example: 0.8 for 1 tier deviation, 0.5 for 2 tiers, etc.
            // This is a simplified scoring, can be made more sophisticated
            (1.0 - (deviation * 0.2)).max(0.0) // Score can't go below 0.0
        }
    }
}