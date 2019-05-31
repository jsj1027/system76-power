use std::{fs, path::Path};

const PERSISTABLE_POWER_PROFILE: &str = "/var/lib/system76-power/power-profile";

/// Values which may persist between daemon reloads and system boots.
pub struct Persistables {
    power_profile: &'static str,
}

impl Persistables {
    pub fn new() -> Self {
        if let Some(default) = Self::create_if_missing() {
            return default;
        }

        if let Ok(data) = fs::read_to_string(PERSISTABLE_POWER_PROFILE) {
            let power_profile = match data.trim() {
                "Balanced" => "Balanced",
                "Battery" => "Battery",
                "Performance" => "Performance",
                _ => {
                    eprintln!("custom power profiles are not yet supported");
                    "Balanced"
                }
            };

            return Self { power_profile };
        }

        Self::default()
    }

    pub fn power_profile(&self) -> &'static str { self.power_profile }

    pub fn set_power_profile(&mut self, power_profile: &'static str) {
        self.power_profile = power_profile;
    }

    pub fn persist(&self) {
        let _ = fs::write(PERSISTABLE_POWER_PROFILE, self.power_profile.as_bytes());
    }

    fn create_if_missing() -> Option<Self> {
        if !Path::new(PERSISTABLE_POWER_PROFILE).exists() {
            let _ = fs::create_dir_all("/var/lib/system76-power/");
            let default = Self::default();
            default.persist();
            return Some(default);
        }

        None
    }
}

impl Default for Persistables {
    fn default() -> Self { Self { power_profile: "Balanced" } }
}
