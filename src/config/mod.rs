use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub theme: String,
    pub width: u32,
    pub height: u32,
    pub font_size: u16,
    pub max_entries: usize,
    pub terminal: String,
    pub search_paths: Vec<PathBuf>,
    pub hide_on_lose_focus: bool,
    pub case_sensitive: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
            width: 900,
            height: 600,
            font_size: 14,
            max_entries: 15,
            terminal: "xterm".to_string(),
            search_paths: vec!["/usr/bin".into(), "/usr/local/bin".into()],
            hide_on_lose_focus: true,
            case_sensitive: false,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .map(|d| d.join("5menu"))
            .unwrap_or_else(|| PathBuf::from("config"));
        
        let config_file = config_dir.join("config.toml");
        
        if !config_file.exists() {
            std::fs::create_dir_all(&config_dir)?;
            let default_config = Self::default();
            let toml = toml::to_string_pretty(&default_config)?;
            std::fs::write(&config_file, toml)?;
            return Ok(default_config);
        }

        let content = std::fs::read_to_string(config_file)?;
        Ok(toml::from_str(&content)?)
    }

    pub fn save(&self) -> Result<()> {
        let config_dir = dirs::config_dir()
            .map(|d| d.join("5menu"))
            .unwrap_or_else(|| PathBuf::from("config"));
        
        std::fs::create_dir_all(&config_dir)?;
        let config_file = config_dir.join("config.toml");
        let toml = toml::to_string_pretty(&self)?;
        std::fs::write(config_file, toml)?;
        Ok(())
    }
}
