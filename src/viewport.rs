use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            width: 1440,
            height: 900,
        }
    }
}

#[derive(Debug, Error)]
pub enum ViewportParseError {
    #[error("Invalid viewport format: expected WIDTHxHEIGHT (e.g., 1440x900)")]
    InvalidFormat,
    #[error("Invalid width: {0}")]
    InvalidWidth(String),
    #[error("Invalid height: {0}")]
    InvalidHeight(String),
    #[error("Width must be positive")]
    ZeroWidth,
    #[error("Height must be positive")]
    ZeroHeight,
}

impl FromStr for Viewport {
    type Err = ViewportParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('x').collect();
        if parts.len() != 2 {
            return Err(ViewportParseError::InvalidFormat);
        }

        let width: u32 = parts[0]
            .trim()
            .parse()
            .map_err(|_| ViewportParseError::InvalidWidth(parts[0].to_string()))?;

        let height: u32 = parts[1]
            .trim()
            .parse()
            .map_err(|_| ViewportParseError::InvalidHeight(parts[1].to_string()))?;

        if width == 0 {
            return Err(ViewportParseError::ZeroWidth);
        }
        if height == 0 {
            return Err(ViewportParseError::ZeroHeight);
        }

        Ok(Viewport { width, height })
    }
}

impl std::fmt::Display for Viewport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid() {
        let vp: Viewport = "1440x900".parse().unwrap();
        assert_eq!(vp.width, 1440);
        assert_eq!(vp.height, 900);
    }

    #[test]
    fn test_parse_with_spaces() {
        let vp: Viewport = " 1920 x 1080 ".parse().unwrap();
        assert_eq!(vp.width, 1920);
        assert_eq!(vp.height, 1080);
    }

    #[test]
    fn test_parse_invalid_format() {
        assert!("1440".parse::<Viewport>().is_err());
        assert!("1440x900x600".parse::<Viewport>().is_err());
        assert!("x900".parse::<Viewport>().is_err());
    }

    #[test]
    fn test_parse_invalid_numbers() {
        assert!("abcx900".parse::<Viewport>().is_err());
        assert!("1440xabc".parse::<Viewport>().is_err());
    }

    #[test]
    fn test_parse_zero_dimensions() {
        assert!("0x900".parse::<Viewport>().is_err());
        assert!("1440x0".parse::<Viewport>().is_err());
    }

    #[test]
    fn test_default() {
        let vp = Viewport::default();
        assert_eq!(vp.width, 1440);
        assert_eq!(vp.height, 900);
    }

    #[test]
    fn test_display() {
        let vp = Viewport {
            width: 1920,
            height: 1080,
        };
        assert_eq!(format!("{}", vp), "1920x1080");
    }
}
