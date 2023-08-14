mod game;

use game::*;

fn main() {
    let game = Presenter::new();

    match game.run() {
        Ok(_) => (),
        Err(s) => println!("File: {}\nLine: {}\nError: {}, exiting...", s.filename, s.line, s.err)
    };
}