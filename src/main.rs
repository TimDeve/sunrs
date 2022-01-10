extern crate sunrs;

use std::env;

use sunrs::messages::{SetColorMessage, SetPowerMessage};
use sunrs::net::send_message;
use sunrs::scenes::{BRIGHT, CHILL, COMPUTER, DARK, DAYLIGHT, READING};

const COMMAND_LINE_USAGE_ERROR_EXIT_CODE: i32 = 64;

fn main() {
    let args: Vec<String> = env::args().collect();

    sunrs::config::get_config();

    match args.get(1) {
        Some(s) => match s.as_ref() {
            "on" => send_message(SetPowerMessage::new_on_message()),
            "off" => send_message(SetPowerMessage::new_off_message()),
            "day" => {
                send_message(SetColorMessage::new_scene(DAYLIGHT));
                send_message(SetPowerMessage::new_on_message());
            }
            "comp" => {
                send_message(SetColorMessage::new_scene(COMPUTER));
                send_message(SetPowerMessage::new_on_message());
            }
            "bright" => {
                send_message(SetColorMessage::new_scene(BRIGHT));
                send_message(SetPowerMessage::new_on_message());
            }
            "read" => {
                send_message(SetColorMessage::new_scene(READING));
                send_message(SetPowerMessage::new_on_message());
            }
            "dark" => {
                send_message(SetColorMessage::new_scene(DARK));
                send_message(SetPowerMessage::new_on_message());
            }
            "chill" => {
                send_message(SetColorMessage::new_scene(CHILL));
                send_message(SetPowerMessage::new_on_message());
            }
            s => print_error_and_exit(
                &format!("Unrecognized command '{}'", s),
                COMMAND_LINE_USAGE_ERROR_EXIT_CODE,
            ),
        },
        None => print_error_and_exit("Need command", COMMAND_LINE_USAGE_ERROR_EXIT_CODE),
    }
}

fn print_error_and_exit(s: &str, exit_code: i32) {
    eprintln!("{}", s);
    std::process::exit(exit_code);
}
