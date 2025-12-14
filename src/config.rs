use std::time::Duration;

use crate::Viewport;

#[derive(Debug, Clone)]
pub struct Config {
    pub viewport: Viewport,
    pub threshold: f64,
    pub metric_weights: MetricWeights,
    pub timeouts: Timeouts,
}

#[derive(Debug, Clone)]
pub struct MetricWeights {
    pub pixel: f32,
    pub layout: f32,
    pub typography: f32,
    pub color: f32,
    pub content: f32,
}

impl Default for MetricWeights {
    fn default() -> Self {
        Self {
            pixel: 1.0,
            layout: 1.0,
            typography: 1.0,
            color: 1.0,
            content: 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Timeouts {
    pub navigation: Duration,
    pub network_idle: Duration,
}

impl Default for Timeouts {
    fn default() -> Self {
        Self {
            navigation: Duration::from_secs(30),
            network_idle: Duration::from_secs(10),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            viewport: Viewport::default(),
            threshold: 0.95,
            metric_weights: MetricWeights::default(),
            timeouts: Timeouts::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, MetricWeights, Timeouts};
    use crate::Viewport;
    use std::time::Duration;

    #[test]
    fn default_values_match_expected() {
        let cfg = Config::default();

        assert_eq!(cfg.viewport.width, 1440);
        assert_eq!(cfg.viewport.height, 900);
        assert!((cfg.threshold - 0.95).abs() < f64::EPSILON);
        assert!((cfg.metric_weights.pixel - 1.0).abs() < f32::EPSILON);
        assert!((cfg.metric_weights.layout - 1.0).abs() < f32::EPSILON);
        assert_eq!(cfg.timeouts.navigation, Duration::from_secs(30));
        assert_eq!(cfg.timeouts.network_idle, Duration::from_secs(10));
    }

    #[test]
    fn can_override_weights_and_timeouts() {
        let cfg = Config {
            viewport: Viewport {
                width: 1280,
                height: 720,
            },
            threshold: 0.9,
            metric_weights: MetricWeights {
                pixel: 0.5,
                layout: 1.2,
                typography: 1.0,
                color: 0.8,
                content: 0.7,
            },
            timeouts: Timeouts {
                navigation: Duration::from_secs(20),
                network_idle: Duration::from_secs(5),
            },
        };

        assert_eq!(cfg.viewport.width, 1280);
        assert_eq!(cfg.viewport.height, 720);
        assert!((cfg.threshold - 0.9).abs() < f64::EPSILON);
        assert!((cfg.metric_weights.pixel - 0.5).abs() < f32::EPSILON);
        assert!((cfg.metric_weights.layout - 1.2).abs() < f32::EPSILON);
        assert_eq!(cfg.timeouts.navigation, Duration::from_secs(20));
        assert_eq!(cfg.timeouts.network_idle, Duration::from_secs(5));
    }
}
