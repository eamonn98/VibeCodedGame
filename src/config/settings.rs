use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct AppConfig {
    pub window_title: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window_title: "VibeCoded Game".to_string(),
        }
    }
}
