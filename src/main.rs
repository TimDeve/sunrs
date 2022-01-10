extern crate sunrs;

use std::env;

use sunrs::messages::{Command, SetColorMessage, SetPowerMessage};
use sunrs::net::send_message;

const COMMAND_LINE_USAGE_ERROR_EXIT_CODE: i32 = 64;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = sunrs::config::Config::from_default_file();

    match args.get(1) {
        Some(s) => match s.as_ref() {
            "on" => send_on_message(config.bulbs_addresses()),
            "off" => send_off_message(config.bulbs_addresses()),
            s => match config.scene_by_command(s) {
                Some(scene) => send_scene_messages(scene),
                None => print_error_and_exit(
                    &format!("Unrecognized command '{}'", s),
                    COMMAND_LINE_USAGE_ERROR_EXIT_CODE,
                ),
            },
        },
        None => print_error_and_exit("Need command", COMMAND_LINE_USAGE_ERROR_EXIT_CODE),
    }
}

fn send_on_message(addresses: Vec<[u8; 8usize]>) {
    for address in addresses.iter() {
        send_message(SetPowerMessage::new_on_message(*address))
    }
}

fn send_off_message(addresses: Vec<[u8; 8usize]>) {
    for address in addresses.iter() {
        send_message(SetPowerMessage::new_off_message(*address))
    }
}

fn send_scene_messages(scene: Vec<Command>) {
    for command in scene.iter() {
        send_message(SetColorMessage::new_scene(*command));
        send_message(SetPowerMessage::new_on_message(command.mac_address))
    }
}

fn print_error_and_exit(s: &str, exit_code: i32) {
    eprintln!("{}", s);
    std::process::exit(exit_code);
}
