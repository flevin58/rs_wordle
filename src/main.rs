use game::Game;

mod assets;
mod game;
mod config;

fn main() {

    let mut game = Game::new();

    let (mut rl, thread) = raylib::init()
        .size(game.width, game.height)
        .title(&game.cfg.window_title)
        .build();

        rl.set_target_fps(game.cfg.fps);

        while !rl.window_should_close() {
            game.update(&mut rl);
            let mut d = rl.begin_drawing(&thread);
            game.draw(&mut d);
        }   
}
