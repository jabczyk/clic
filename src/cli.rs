use crate::colors::Colors;
use crate::constants::{COMMAND_COLOR, COMMAND_CONSTS, COMMAND_HELP, COMMAND_SET};
use crate::context::Context;
use crate::store::get_config_path;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Cli {
    context: Context,
    colors: Colors,
}

impl Cli {
    pub fn new() -> Self {
        Self {
            context: Context::build(),
            colors: Colors::build(),
        }
    }

    pub fn execute_command(&mut self, arguments: &[String]) {
        if arguments.is_empty() {
            self.enter_shell_mode();
            return;
        }

        match arguments[0].as_str() {
            COMMAND_HELP => self.print_help(arguments.get(1).map(String::as_str)),
            COMMAND_SET => self.set_constant(&arguments[1..]),
            COMMAND_CONSTS => self.print_constants(),
            COMMAND_COLOR => self.set_color(&arguments[1..]),
            expression => self.evaluate_expression(expression),
        }
    }

    fn enter_shell_mode(&mut self) {
        let mut rl = Editor::<()>::new();

        let history_path = get_config_path("history.txt");
        if rl.load_history(&history_path).is_err() {}

        loop {
            let readline = rl.readline("> ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());

                    let arguments = Self::parse_shell_arguments(line);
                    self.execute_command(&arguments);
                }
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
                Err(err) => {
                    self.colors
                        .print_fail(format!("Unexpected error: {:?}", err));
                    break;
                }
            }
        }

        rl.save_history(&history_path).unwrap();
    }
    fn parse_shell_arguments(line: String) -> Vec<String> {
        let arguments: Vec<String> = line.split_whitespace().map(String::from).collect();

        // allow whitespace in expressions, without the quotation marks
        match arguments.get(0).map(String::as_str) {
            None => vec![],
            Some(COMMAND_HELP) | Some(COMMAND_SET) | Some(COMMAND_COLOR) => arguments,
            Some(_) => vec![line],
        }
    }

    fn print_help(&self, topic: Option<&str>) {
        match topic {
            Some(_) | None => {
                println!(
                    r#"{heading}

{basic_usage}
    clic - enter shell mode
    clic "sqrt(3)" - evaluate a math expresssion
    clic help [topic] - view help

{constants}
    clic set <name> <value> - create a custom constant
    clic consts - view custom constants

{colors}
    clic color <primary/secondary/failure> <color> - set a color
    ~ For color names, please refer to {colors_link}"#,
                    heading = self
                        .colors
                        .primary(String::from("Clic - A simple CLI calculator")),
                    basic_usage = self.colors.secondary(String::from("Basic usage")),
                    constants = self.colors.secondary(String::from("Constants")),
                    colors = self.colors.secondary(String::from("Colors")),
                    colors_link = self.colors.secondary(String::from("https://github.com/mackwic/colored#colors"))
                );
            }
        }
    }

    fn set_constant(&mut self, arguments: &[String]) {
        let value = match arguments[1].parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                println!("Error: Invalid constant value");
                return;
            }
        };

        self.context.set_constant(arguments[0].to_owned(), value);
    }

    fn print_constants(&self) {
        println!("{}", self.colors.primary(String::from("Custom constants")));
        for (constant, value) in self.context.get_constants() {
            println!("    {} = {}", constant, value);
        }
        println!(
            "For built-in constants, please refer to {link}",
            link = self.colors.secondary(String::from(
                "https://github.com/rekka/meval-rs#supported-expressions"
            ))
        );
    }

    fn set_color(&mut self, arguments: &[String]) {
        let value = match arguments.get(1) {
            Some(v) => v,
            None => {
                self.colors
                    .print_fail("Usage: color <primary/secondary/failure> <color>".to_owned());
                return;
            }
        };

        self.colors.set(&arguments[0], value.to_owned());
    }

    fn evaluate_expression(&self, expression: &str) {
        match meval::eval_str_with_context(expression, self.context.get()) {
            Ok(result) => println!("{}", self.colors.primary(format!("= {}", result))),
            Err(message) => self.colors.print_fail(format!("{}", message)),
        }
    }
}
