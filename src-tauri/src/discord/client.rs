use crate::models::config::{ActivityTypeConfig, AppConfig, ButtonConfig, StatusDisplayTypeConfig};
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use std::{
    error::Error,
    fs::{self, File},
    path::Path,
};

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            application_id: Some("1402418436809953330".to_string()),
            activity_type: Some(ActivityTypeConfig::Playing),
            details: Some("{{nickname}} | Lv. {{level}}".to_string()),
            state: Some("HP: {{hp}}/{{max_hp}} | {{fp}}/{{max_fp}} | {{sp}}/{{max_sp}}".to_string()),
            buttons: Some(vec![ButtonConfig {
                label: "Download Now!".to_string(),
                url: "https://github.com/rushkii/EldRingPresence".to_string(),
            }]),
            status_display_type: Some(StatusDisplayTypeConfig::Name),
        }
    }
}

pub struct DiscordRPC {
    client: DiscordIpcClient,
    pub connected: bool,
    pub config: AppConfig,
}

impl DiscordRPC {
    pub fn new() -> Self {
        let config = Self::verify_config();
        let client_id = &config.application_id.clone().unwrap_or_default();
        let client = DiscordIpcClient::new(client_id);

        Self {
            client,
            connected: false,
            config,
        }
    }

    pub fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        self.client.connect()?;
        self.connected = true;
        eprintln!("Connected to Discord RPC.");
        Ok(())
    }

    pub fn disconnect(&mut self) -> Result<(), Box<dyn Error>> {
        if self.connected {
            self.client.close()?;
            self.connected = false;
            eprintln!("Disconnected to Discord RPC.");
        }
        Ok(())
    }

    pub fn set_activity(
        &mut self,
        activity: discord_rich_presence::activity::Activity,
    ) -> Result<(), Box<dyn Error>> {
        if !self.connected {
            return Err("Discord RPC not connected".into());
        }

        self.client.set_activity(activity)?;

        Ok(())
    }

    fn verify_config() -> AppConfig {
        let path = Path::new("./config.json");

        if !path.exists() {
            println!("Config not found, creating default...");
            return Self::populate_config(path);
        }

        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return Self::populate_config(path),
        };

        match serde_json::from_str::<AppConfig>(&content) {
            Ok(valid_config) => {
                println!("Config loaded successfully.");
                valid_config
            }
            Err(e) => {
                eprintln!("Invalid JSON syntax detected: {}", e);
                eprintln!("Backing up config.json to config.old.json and resetting defaults.");

                let backup_path = Path::new("./config.old.json");

                let _ = fs::copy(path, backup_path);

                Self::populate_config(path)
            }
        }
    }

    fn populate_config(path: &Path) -> AppConfig {
        let default_config = AppConfig::default();

        match File::create(path) {
            Ok(file) => {
                if let Err(e) = serde_json::to_writer_pretty(file, &default_config) {
                    eprintln!("Failed to write config file: {}", e);
                }
            }
            Err(e) => eprintln!("Failed to create config file: {}", e),
        }

        default_config
    }
}
