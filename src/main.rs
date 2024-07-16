use arboard::Clipboard;
use std::env::args;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut clipboard = Clipboard::new().unwrap();
    if atty::is(atty::Stream::Stdin) {
        if args().len() > 1 {
            let arg = args().nth(1).unwrap();
            if (arg == "-h") || (arg == "--help") || (arg == "help") {
                print_help();
            }
            return Ok(());
        }

        let contents = clipboard.get_text().unwrap();
        print!("{}", contents);
        return Ok(());
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;

        clipboard.set_text(buffer.to_owned()).unwrap();
        assert_eq!(clipboard.get_text().unwrap(), buffer);
    }

    Ok(())
}

fn print_help() {
    println!("Usage for x\n");
    println!("To copy text to clipboard, use the pipe operator");
    println!("$ echo 'Hello, World!' | x\n");
    println!("To paste text from clipboard, use the x command without any arguments");
    println!("$ x");
    println!("# Hello, World!");
}
