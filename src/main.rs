use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    // the first argument is the executable path, it can be ignored
    execute_command(&arguments[1..]);
}

fn execute_command(arguments: &[String]) {
    if arguments.is_empty() {
        panic!("Shell mode is not yet supported");
    }

    match arguments[0].as_str() {
        "help" => print_help(arguments.get(1).map(String::as_str)),
        expression => evaluate_expression(expression),
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
