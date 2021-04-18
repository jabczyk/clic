use crate::store::{load_from_json, persist_json};
use colored::*;
use serde::{Deserialize, Serialize};

const COLORS_FILE: &str = "colors.json";

#[derive(Serialize, Deserialize)]
pub struct Colors {
    primary: String,
    secondary: String,
    failure: String,
}

impl Colors {
    pub fn build() -> Self {
        match load_from_json::<Self>(COLORS_FILE) {
            Ok(ctx) => ctx,
            Err(_) => {
                Self {
                    primary: String::from("cyan"),
                    secondary: String::from("yellow"),
                    failure: String::from("red"),
                }
            }
        }
    }

    fn get(&self, key: &str) -> Color {
        let color_string = match key {
            "primary" => &self.primary,
            "secondary" => &self.secondary,
            "failure" => &self.failure,
            _ => "white",
        };
        let color_res: Result<Color, ()> = color_string.parse();

        color_res.unwrap_or(Color::White)
    }

    pub fn set(&mut self, key: &str, val: String) {
        match key {
            "primary" => self.primary = val,
            "secondary" => self.secondary = val,
            "failure" => self.failure = val,
            _ => {
                self.print_fail("Invalid key, available: primary, secondary, failure".to_owned());
                return;
            }
        };

        persist_json(COLORS_FILE, self);
    }

    pub fn primary(&self, text: String) -> ColoredString {
        text.color(self.get("primary"))
    }

    pub fn secondary(&self, text: String) -> ColoredString {
        text.color(self.get("secondary"))
    }

    pub fn print_fail(&self, text: String) {
        println!("{}", text.color(self.get("failure")))
    }
}
