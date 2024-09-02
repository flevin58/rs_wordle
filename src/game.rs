use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 480;
const FPS: u32 = 60;

pub struct Game {
    rl: RaylibHandle,
    thread: RaylibThread,
}

impl Game {
    pub fn new() -> Self {
        let (mut r, t) = raylib::init().build();

        r.set_target_fps(FPS);
        r.set_window_size(WINDOW_WIDTH, WINDOW_HEIGHT);
        r.set_exit_key(Some(KEY_F12));


        Game{
            rl: r,
            thread: t,
        }
    }

    pub fn update(&self) {
    }

    pub fn draw(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(Color::BLACK);

        d.draw_circle(100, 100, 50.0, Color::GREEN);
        d.draw_text("Press F12 to exit", 20, WINDOW_HEIGHT-30, 24, Color::WHITE);
    }

    pub fn has_finished(&self) -> bool{
        self.rl.window_should_close()
    }
}
