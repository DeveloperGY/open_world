use std::cell::RefCell;

#[derive(Clone, Copy)]
pub enum State {
    MainMenu,
    Game,
    Exit
}

pub struct Model {
    state: RefCell<State>,
    app_running: RefCell<bool>
}

impl Model {
    pub fn new() -> Self {
        let state = RefCell::new(State::MainMenu);
        let app_running = RefCell::new(false);

        Self {
            state,
            app_running
        }
    }

    pub fn get_state(&self) -> State {
        *self.state.borrow()
    }

    pub fn set_state(&self, state: State) {
        *self.state.borrow_mut() = state;
    }

    pub fn app_running(&self) -> bool {
        *self.app_running.borrow()
    }

    pub fn set_app_running(&self, running: bool) {
        *self.app_running.borrow_mut() = running;
    }
}