use macroquad::color::Color;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WheelChoice {
    pub name: String,
    pub desc: Option<String>,
}

impl WheelChoice {
    pub fn new(name: &str, desc: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            desc: desc.map(|desc| desc.to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wheel {
    /// The wheel's raw hex code color pallete.
    pallete: Vec<String>,
    /// Name or path to the font the wheel uses.
    pub font: String,
    pub choices: Vec<WheelChoice>,
}

impl Wheel {
    pub fn get_pallete(&self) -> Vec<Color> {
        fn convert_hex(hex_code: &str) -> Color {
            let mut is_valid = true;
            let hex_code = hex_code.trim_start_matches('#');

            if hex_code.len() != 6 {
                is_valid = false
            }
            let (r, g, b) = (
                u8::from_str_radix(&hex_code[..2], 16)
                    .inspect_err(|_| is_valid = false)
                    .unwrap(),
                u8::from_str_radix(&hex_code[2..4], 16)
                    .inspect_err(|_| is_valid = false)
                    .unwrap(),
                u8::from_str_radix(&hex_code[4..], 16)
                    .inspect_err(|_| is_valid = false)
                    .unwrap(),
            );

            if is_valid {
                Color::from_rgba(r, g, b, 255)
            } else {
                macroquad::color::PINK
            }
        }

        self.pallete.iter().map(|hex| convert_hex(&hex)).collect()
    }
    /// Attempts to load a wheel from a json file.
    pub fn load(path: impl Into<PathBuf>) -> Result<Self, Error> {
        let path: PathBuf = path.into();

        let wheel_json = std::fs::read_to_string(path)?;

        // for some reason i have to map the error. total bs...
        serde_json::from_str(&wheel_json).map_err(|error| Error::from(error))
    }
}

impl Default for Wheel {
    fn default() -> Self {
        let mut choices = Vec::new();

        choices.push(WheelChoice::new("Yes", None));
        choices.push(WheelChoice::new("No", None));

        Self {
            pallete: vec!["#fb2646".to_string(), "#0077ff".to_string()],
            font: String::from("assets/font/NotoSans.ttf"),
            choices,
        }
    }
}
