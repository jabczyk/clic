mod store;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env;
use store::{create_config_dir, get_history_path};

const COMMAND_HELP: &str = "help";

fn main() {
    create_config_dir();

    let arguments: Vec<String> = env::args().collect();

    // the first argument is the executable path, it can be ignored
    execute_command(&arguments[1..]);
}

fn execute_command(arguments: &[String]) {
    if arguments.is_empty() {
        enter_shell_mode();
        return;
    }

    match arguments[0].as_str() {
        COMMAND_HELP => print_help(arguments.get(1).map(String::as_str)),
        expression => evaluate_expression(expression),
    }
}

fn enter_shell_mode() {
    let mut rl = Editor::<()>::new();

    let history_path = get_history_path();
    if rl.load_history(&history_path).is_err() {}

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                let arguments = parse_shell_arguments(line);
                execute_command(&arguments);
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
        Some(COMMAND_HELP) => arguments,
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
    clic help [topic] - view help"#
            );
        }
    }
}

fn evaluate_expression(expression: &str) {
    println!("evaluated: {}", meval::eval_str(expression).unwrap());
}
