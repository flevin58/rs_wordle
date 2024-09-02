use game::*;

mod game;

fn main() {
    let mut game = Game::new();
    while !game.has_finished() {
        game.update();
        game.draw();
    }
}
