extern crate titanium;

use titanium::game::{self, Game, Game2D};

fn main() {
    let mut game = Game2D::init_game("test app".to_string());
    game.run();
}
