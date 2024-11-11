extern crate winapi;

use winapi::um::wincon::GetConsoleWindow;
use winapi::um::winuser::{GetWindowLongPtrW, SetWindowLongPtrW, GWL_STYLE, WS_MAXIMIZEBOX};
use std::ptr::null_mut;

use std::process::Command;
use std::time::Duration;
use std::{io::{self, Write}, thread::sleep};

use colored::*;
use sysinfo::System;
use terminal_size::{Width, terminal_size};

fn main() {
    banner();
    no_resize();

    print!("\x1b]0; \x07");
    io::stdout().flush().unwrap();

    print!("\x1b[?25l");
    io::stdout().flush().unwrap();

    let starting_msg = set_x_position("          Starting Drip Lite in 10 seconds...", None).red();
    print!("{}", starting_msg);
    io::stdout().flush().unwrap();

    sleep(Duration::from_secs(10));

    let mut system = System::new_all();
    let mut message = set_x_position("          Waiting for Minecraft process...", None).red(); // Mensaje inicial

    loop {
        system.refresh_all();
        let java_process = system.processes().values().any(|p| p.name() == "javaw.exe");

        if java_process {
            message = set_x_position("          Connecting...", None).red();
            print!("\r{}{}", message, " ".repeat(starting_msg.len() - message.len()));
            io::stdout().flush().unwrap();
            sleep(Duration::from_secs(5));

            message = set_x_position("          Injecting...", None).red();
            print!("\r{}{}", message, " ".repeat(starting_msg.len() - message.len()));
            io::stdout().flush().unwrap();

            sleep(Duration::from_secs(10));
            message = set_x_position("          Successfully injected.", None).yellow();
            print!("\r{}{}", message, " ".repeat(starting_msg.len() - message.len()));
            io::stdout().flush().unwrap();
            sleep(Duration::from_secs(5));
            break;
        } else {
            print!("\r{}{}", set_x_position("          Waiting for Minecraft process...", None).red(), " ".repeat(starting_msg.len() - message.len()));
            io::stdout().flush().unwrap();
        }
    }
}

fn set_x_position(text: &str, x: Option<usize>) -> String {
    if let Some((Width(w), _)) = terminal_size() {
        let padding = match x {
            Some(x_pos) => (w as usize).saturating_sub(text.len()) - x_pos,
            None => (w as usize).saturating_sub(text.len()) / 2,
        };
        format!("{}{}", " ".repeat(padding), text)
    } else {
        text.to_string()
    }
}

fn no_resize() {
    let hwnd = unsafe { GetConsoleWindow() };

    if hwnd != null_mut() {
        let style = unsafe { GetWindowLongPtrW(hwnd, GWL_STYLE) };

        unsafe { SetWindowLongPtrW(hwnd, GWL_STYLE, (style & !(WS_MAXIMIZEBOX as isize)) as isize) };
    }
}

fn clear_screen() {
    Command::new("cmd").arg("/C").arg("cls").status().unwrap();
}

fn banner() {
    clear_screen();
    println!("{}", "\n\n\n");
    let ascii_art = r#"
     _      _         _ _ _
       __| |_ __(_)_ __   | (_) |_ ___
       / _` | '__| | '_ \  | | | __/ _ \
      | (_| | |  | | |_) | | | | ||  __/
       \__,_|_|  |_| .__/  |_|_|\__\___|
 |_|
    "#;
    
    for line in ascii_art.lines() {
        println!("{}", set_x_position(line, None).custom_color(CustomColor::new(235, 0, 255)));
    }
    
    println!("{}", set_x_position("v3.2", Some(35)).bright_black());
    println!("{}", "\n\n\n\n\n\n\n\n\n\n\n");
}
