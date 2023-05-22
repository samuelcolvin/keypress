use std::env;
use std::process::exit;
use enigo::*;

fn main() {
    let mut enigo = Enigo::new();

    let mut args = env::args().skip(1).collect::<Vec<String>>();

    let debug = match args.iter().position(|s| s == "--debug") {
        Some(index) => {
            args.remove(index);
            true
        },
        None => false,
    };
    let mut actions = 0;

    for arg in env::args().skip(1) {
        match act(&mut enigo, &arg) {
            Ok(s) => {
                actions += 1;
                if debug {
                    println!("{}", s)
                }
            },
            Err(e) => {
                println!("Error: {}", e);
                exit(1)
            },
        }
    }
    if actions == 0 {
        eprintln!("Usage: [--debug] [action] [action] ...");
        exit(1)
    }
}

fn act(enigo: &mut Enigo, arg: &str) -> Result<String, String> {
    if let Some(key_name) = arg.strip_prefix("down:") {
        let key = get_key(key_name)?;
        enigo.key_down(key);
        Ok(format!("Key down: {key:?}"))
    } else if let Some(key_name) = arg.strip_prefix("up:") {
        let key = get_key(key_name)?;
        enigo.key_up(key);
        Ok(format!("Key up: {key:?}"))
    } else {
        let key = get_key(arg)?;
        enigo.key_click(key);
        Ok(format!("press: {key:?}"))
    }
}


fn get_key(key_name: &str) -> Result<Key, String> {
    match key_name {
        "ctrl" => Ok(Key::Control),
        "shift" => Ok(Key::Shift),
        "alt" => Ok(Key::Alt),
        "enter" => Ok(Key::Return),
        "backspace" => Ok(Key::Backspace),
        "tab" => Ok(Key::Tab),
        "capslock" => Ok(Key::CapsLock),
        "esc" => Ok(Key::Escape),
        "space" => Ok(Key::Space),
        "pageup" => Ok(Key::PageUp),
        "pagedown" => Ok(Key::PageDown),
        "end" => Ok(Key::End),
        "home" => Ok(Key::Home),
        otherwise => {
            if otherwise.len() == 1 {
                Ok(Key::Layout(otherwise.chars().next().unwrap()))
            } else {
                Err(format!("Invalid key: {}", otherwise))
            }
        }
    }
}
