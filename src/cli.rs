use crate::constants::{COMMAND_CONSTS, COMMAND_HELP, COMMAND_SET};
use crate::context::Context;
use crate::store::get_config_path;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Cli {
    context: Context,
}

impl Cli {
    pub fn new() -> Self {
        Self {
            context: Context::build(),
        }
    }

    pub fn execute_command(&mut self, arguments: &[String]) {
        if arguments.is_empty() {
            self.enter_shell_mode();
            return;
        }

        match arguments[0].as_str() {
            COMMAND_HELP => Self::print_help(arguments.get(1).map(String::as_str)),
            COMMAND_SET => self.set_constant(&arguments[1..]),
            COMMAND_CONSTS => self.print_constants(),
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
                    println!("Oops, something went wrong! Error: {:?}", err);
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
            Some(COMMAND_HELP) | Some(COMMAND_SET) => arguments,
            Some(_) => vec![line],
        }
    }

    fn print_help(topic: Option<&str>) {
        match topic {
            Some(_) | None => {
                println!(
                    r#"Clic - A simple CLI calculator

Basic usage
    clic - enter shell mode
    clic "sqrt(3)" - evaluate a math expresssion
    clic help [topic] - view help

Constants
    clic set <name> <value> - create a custom constant
    clic consts - view custom constants"#
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
        println!("Custom constants");
        for (constant, value) in self.context.get_constants() {
            println!("    {} = {}", constant, value);
        }
        println!("For built-in constants, please refer to https://github.com/rekka/meval-rs#supported-expressions");
    }

    fn evaluate_expression(&self, expression: &str) {
        match meval::eval_str_with_context(expression, self.context.get()) {
            Ok(result) => println!("= {}", result),
            Err(message) => println!("Oops! {}", message),
        }
    }
}
