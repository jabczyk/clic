use colored::*;

pub struct Colors {
    primary: String,
    secondary: String,
    failure: String,
}

impl Colors {
    pub fn build() -> Self {
        Self {
            primary: String::from("cyan"),
            secondary: String::from("yellow"),
            failure: String::from("red"),
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
