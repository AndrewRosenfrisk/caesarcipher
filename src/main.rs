use clipboard_ext::prelude::*;
use clipboard_ext::x11_fork::ClipboardContext;

use std::fmt;

#[derive(PartialEq, Debug)]
enum Mode {
    Encrypt,
    Decrypt,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mode::Encrypt => write!(f, "encrypt",),
            Mode::Decrypt => write!(f, "decrypt",),
        }
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let symbols = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut max_key = symbols.chars().count().try_into()?;
    max_key -= 1;

    println!("Do you want to (e)ncrypt or (d)ecrypt?: ");

    let mode: Mode;
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();

        let option = buf
            .trim()
            .to_string()
            .to_uppercase()
            .chars()
            .nth(0)
            .unwrap();

        println!("Output: {}", option);

        match option {
            'E' => {
                mode = Mode::Encrypt;
                break;
            }
            'D' => {
                mode = Mode::Decrypt;
                break;
            }
            _ => {
                println!("Invalid option.");
                continue;
            }
        }
    }

    let mut key: usize;
    loop {
        let mut buf = String::new();
        println!(
            "Enter an integer from 1 through {} to use as a key",
            max_key
        );
        std::io::stdin().read_line(&mut buf).unwrap();

        key = buf.trim().parse::<usize>().unwrap_or(0);

        if key > 0 && key <= max_key {
            break;
        } else {
            println!("Invalid integer.");
            continue;
        }
    }
    println!("Enter the message to {}:", mode);
    let mut buf = String::new();

    std::io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read input");

    let message = buf.trim().to_string().to_uppercase();

    let mut translation: String = String::new();

    for char in message.char_indices() {
        if symbols.contains(&char.1.to_string()) {
            let mut num: isize = symbols.find(char.1).unwrap().try_into()?;

            match mode {
                Mode::Encrypt => num += key as isize,
                Mode::Decrypt => num -= key as isize,
            }

            if num >= symbols.chars().count() as isize {
                num -= symbols.chars().count() as isize;
            } else if num < 0 {
                num += symbols.chars().count() as isize;
            }

            translation.push(symbols.chars().nth(num.try_into()?).unwrap());
        } else {
            translation.push(char.1);
        }
    }
    println!("{}", translation);

    let mut ctx = ClipboardContext::new().unwrap();

    println!("Full {}ed text copied to clipboard.", mode);

    ctx.set_contents(translation).unwrap();

    Ok(())
}
