use crate::window::TitaniumWindow;


pub trait Game {
    fn run(&mut self);
    fn init_game(title: String) -> Self;
}

pub struct Game2D {
    pub window: TitaniumWindow,
}

impl Game for Game2D {
    fn init_game(title: String) -> Self {
        let game = Game2D {
            window: TitaniumWindow::init(title)
        };
        game
    }

    fn run(&mut self) {
        self.window.run();
    }
}