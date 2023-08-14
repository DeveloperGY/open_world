use std::io::Write;

use super::gm;


const MIN_UNDERLINE: usize = 15;


pub struct View;

impl View {
    pub fn print_title(title: &str) {
        let title_len = title.chars().collect::<Vec<_>>().len();
        let underline_len = if title_len < MIN_UNDERLINE {MIN_UNDERLINE} else {title_len};

        let underline = (0..underline_len).map(|_| {"="}).collect::<String>();
        println!("{}\n{}", title, underline);
    }

    pub fn print_options(options: &[&str]) {
        for option in options {
            println!("- {}", *option);
        }
    }

    pub fn clear_screen() -> gm::Result<()> {
        print!("\x1b[H\x1b[2J");

        match std::io::stdout().flush() {
            Ok(_) => Ok(()),
            Err(_) => {
                let mut err = gm::Error::new();
                err.set_err("failed to clear screen");
                Err(err)
            }
        }
    }

    pub fn main_menu(options: &[&str]) -> gm::Result<String> {
        View::clear_screen()?;
        View::print_title("Main Menu");
        View::print_options(options);
        View::prompt("")
    }
}

impl View {
    pub fn prompt(prompt: &str) -> gm::Result<String> {
        print!("{}", prompt);
        match std::io::stdout().flush() {
            Ok(_) => (),
            Err(_) => {
                let mut err = gm::Error::new();
                err.set_err("failed to prompt user input");
                return Err(err);
            }
        };


        let mut buf = String::new();

        match std::io::stdin().read_line(&mut buf) {
            Ok(_) => Ok(buf),
            Err(_) => {
                let mut err = gm::Error::new();
                err.set_err("failed to retrieve user input");
                Err(err)
            }
        }
    }
}