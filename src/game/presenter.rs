use super::{Model, View, gm};
use super::{State, gm::gmerr};

pub struct Presenter {
    model: Model
}

impl Presenter {
    pub fn new() -> Self {
        let model = Model::new();
        Self {
            model
        }
    }

    pub fn run(&self) -> gm::Result<()> {
        self.model.set_app_running(true);

        while self.model.app_running() {
            gmerr!(View::clear_screen());

            match self.model.get_state() {
                State::MainMenu => {
                    self.main_menu()?;
                },
                State::Game => {
                    self.game()?;
                },
                State::Exit => {
                    self.exit();
                }
            }
        }

        Ok(())
    }

    fn main_menu(&self) -> gm::Result<()> {
        let menu_options = ["start", "exit"]; // change to enum with string associated values that view can access


        let input = gmerr!(View::main_menu(&menu_options)).to_lowercase();
        let input = input.trim();

        match input {
            "start" => {
                self.model.set_state(State::Game);
            },
            "exit" => {
                self.model.set_state(State::Exit);
            }
            _ => (),
        };

        Ok(())
    }

    fn exit(&self) {
        self.model.set_app_running(false);
    }

    fn game(&self) -> gm::Result<()> {
        gmerr!(View::prompt("Game started, press enter to exit..."));
        self.model.set_state(State::Exit);
        Ok(())
    }
}