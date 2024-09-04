use raylib::color::Color;
use serde_derive::Deserialize;
use toml;
use crate::assets;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub window_title: String,
    pub fps: u32,
    pub geometry: Geometry,
    pub colors: GameColors,
    pub messages: Messages,
    pub fonts: GameFonts,
}

#[derive(Deserialize, Debug)]
pub struct Geometry {
    pub nrows: i32,
    pub ncols: i32,
    pub titleh: i32,
    pub boxsz: i32,
    pub boxsp: i32,
    pub statush: i32,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub title: String,
    pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct Messages {
    pub playing: Message,
    pub enter: Message,
    pub win: Message,
    pub loose: Message,
}

#[derive(Deserialize, Debug)]
pub struct GameColors {
    pub titlearea: String,
    pub gridarea: String,
    pub statusarea: String,
    pub outline: String,
    pub emptybox: String,
    pub noletters: String,
    pub wrongpos: String,
    pub rightpos: String,
    pub answer: String,
}

#[derive(Deserialize, Debug)]
pub struct GameFonts {
    pub wordle: GameFont,
    pub letter: GameFont,
    pub title: GameFont,
    pub message: GameFont,
}

#[derive(Deserialize, Debug)]
pub struct GameFont {
    pub face: String,
    pub size: i32,
}

pub fn load_config() -> Config {
    let toml_string = assets::get_toml_data();
    let cfg = toml::from_str(&toml_string).unwrap();
    cfg
}

impl Config {
    pub fn get_color(&self, which: &str) -> Color {
        match which {
        "title" => return Color::from_hex(self.colors.titlearea.as_str()).unwrap(),
        "grid" => return Color::from_hex(self.colors.gridarea.as_str()).unwrap(),
        "status" => return Color::from_hex(self.colors.statusarea.as_str()).unwrap(),
        "outline" => return Color::from_hex(self.colors.outline.as_str()).unwrap(),
        "empty" => return Color::from_hex(self.colors.emptybox.as_str()).unwrap(),
        "noletters" => return Color::from_hex(self.colors.noletters.as_str()).unwrap(),
        "right" => return Color::from_hex(self.colors.rightpos.as_str()).unwrap(),
        "wrong" => return Color::from_hex(self.colors.wrongpos.as_str()).unwrap(),
        "answer" => return Color::from_hex(self.colors.answer.as_str()).unwrap(),
        _ => return Color::WHITE,
        }
    }
}

impl Geometry {

    pub fn x_box_coord_at_col(&self, col: i32) -> i32 {
        self.boxsp + col  * (self.boxsp + self.boxsz)
    }
    
    pub fn y_box_coord_at_row(&self, row: i32) -> i32 {
        self.titleh + self.boxsp + row * (self.boxsp + self.boxsz)
    }
}
