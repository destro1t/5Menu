use serde::{Deserialize, Serialize};
use iced::Color;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub background_color: String,
    pub text_color: String,
    pub selected_background_color: String,
    pub selected_text_color: String,
    pub border_color: String,
    pub border_width: f32,
    pub border_radius: f32,
    pub padding: f32,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            background_color: "#2E344000".to_string(),
            text_color: "#ECEFF4FF".to_string(),
            selected_background_color: "#5E81ACFF".to_string(),
            selected_text_color: "#FFFFFFFF".to_string(),
            border_color: "#4C566A00".to_string(),
            border_width: 0.0,
            border_radius: 0.0,
            padding: 10.0,
        }
    }
}

impl Theme {
    pub fn load(name: &str) -> Result<Self> {
        let theme_dir = dirs::config_dir()
            .map(|d| d.join("5menu").join("themes"))
            .unwrap_or_else(|| PathBuf::from("config/themes"));
        
        let theme_file = theme_dir.join(format!("{}.toml", name));
        
        if !theme_file.exists() {
            if name == "default" {
                let default_theme = Self::default();
                std::fs::create_dir_all(&theme_dir)?;
                let toml = toml::to_string_pretty(&default_theme)?;
                std::fs::write(&theme_file, toml)?;
                return Ok(default_theme);
            }
            anyhow::bail!("Theme {} not found", name);
        }

        let content = std::fs::read_to_string(theme_file)?;
        Ok(toml::from_str(&content)?)
    }

    pub fn parse_color(&self, hex: &str) -> Color {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
        let a = if hex.len() >= 8 {
            u8::from_str_radix(&hex[6..8], 16).unwrap_or(255)
        } else {
            255
        };
        Color::from_rgba(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a as f32 / 255.0,
        )
    }
}
