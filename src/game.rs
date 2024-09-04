use raylib::prelude::*;
use crate::config::{load_config, Config, Message};
use crate::assets::get_random_word;

#[derive(PartialEq, Eq)]
enum GameStatus {
    Playing,
    Won,
    Lost,
}

pub struct Game {
    pub width: i32,
    pub height: i32,
    pub cfg: Config,
    answer: Vec<u8>,
    status: GameStatus,
    row: i32,
    col: i32,
    grid: Vec<Vec<u8>>,
	ticks: u64,
	blinksecs: usize,
}

impl Game {
    pub fn new() -> Self {

        // load configuration from embedded file "wordle.toml"
        let c = load_config();

        // let's calculate the window size
        let ncols = c.geometry.ncols;
        let nrows = c.geometry.nrows;
        let window_width = ncols * c.geometry.boxsz + (ncols+1) * c.geometry.boxsp;
        let window_height = c.geometry.titleh + nrows * c.geometry.boxsz + (nrows+1) * c.geometry.boxsp + c.geometry.statush;
    
        Game{
            width: window_width,
            height: window_height,
            cfg: c,
            answer: get_random_word(),
            status: GameStatus::Playing,
            row: 0,
            col: 0,
            grid: vec![vec![0; ncols as usize]; nrows as usize],
            ticks: 0,
            blinksecs: 0,
        }
    }

    pub fn reset(&mut self) {
        self.answer = get_random_word();
        self.status = GameStatus::Playing;
        self.row = 0;
        self.col = 0;

        let nrows: usize = self.cfg.geometry.nrows as usize;
        let ncols: usize = self.cfg.geometry.ncols as usize;
        for y in 0..nrows {
            for x in 0..ncols {
                self.grid[y][x] = 0;
            }
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {

        let key_pressed = rl.get_key_pressed();
        match key_pressed {
            None => return,
            Some(key) => match key {
                KeyboardKey::KEY_BACKSPACE => {
                    if self.status == GameStatus::Playing {
                        if self.col < self.cfg.geometry.ncols {
                            self.grid[self.row as usize][self.col as usize] = 0
                        }
                
                        // Move left
                        if self.col > 0 {
                            self.col -= 1;
                        }
                        self.grid[self.row as usize][self.col as usize] = 0
                    }        
                },
                KeyboardKey::KEY_A => {
                    print!("{:?}", key);                    
                }
                _ => {
                    // do nothong, TBD play error sound!
                }

            }
        }
    }

    // Returns the foreground anf background colors for a grid square
    fn get_color(&mut self, row:i32, col:i32) -> (Color, Color) {

        // All lines below and including current have
        // black font on lightgrey background
        if row >= self.row {
            return (Color::BLACK, self.cfg.get_color("empty"));
        }

        // Check proper color
        let grid_char: u8 = self.grid[row as usize][col as usize];

        for (i, char) in self.answer.iter().enumerate() {
            if i == col as usize && *char == grid_char {
                return (Color::WHITE, self.cfg.get_color("right"));
            }
            if *char == grid_char {
                return (Color::WHITE, self.cfg.get_color("wrong"));
            }
        }
        
        // No chars found
        return (Color::WHITE, self.cfg.get_color("noletters"));
    }
        
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::WHITE);

        // Draw the title area and title
        d.draw_rectangle(0, 0, self.width, self.height, self.cfg.get_color("title"));

        let wtitle = self.cfg.window_title.as_str();
        let fsize = self.cfg.fonts.title.size;
        let wtitle_width = d.measure_text(wtitle, fsize);
        let center_x = (self.width - wtitle_width) / 2;
        d.draw_text(wtitle, center_x, self.cfg.geometry.titleh, fsize, Color::BLACK);

        // Draw the grid area
        let ygrid = self.cfg.geometry.titleh;
        let nrows = self.cfg.geometry.nrows;
        let ncols = self.cfg.geometry.ncols;
        let hgrid = (nrows+1) * self.cfg.geometry.boxsp + nrows * self.cfg.geometry.boxsz;
        d.draw_rectangle(0, ygrid, self.width, hgrid, self.cfg.get_color("grid"));

        // Draw current position cursor outline
	    if self.status == GameStatus::Playing && self.col < ncols {
            let y = self.cfg.geometry.titleh + self.row * (self.cfg.geometry.boxsz + self.cfg.geometry.boxsp);
            let x = self.col * (self.cfg.geometry.boxsz + self.cfg.geometry.boxsp);
            let sz = self.cfg.geometry.boxsz + 2 * self.cfg.geometry.boxsp;
            d.draw_rectangle(x, y, sz, sz, self.cfg.get_color("outline"));
        }
            
        // Draw grid
	    for row in 0..nrows {
		    let y_coord = self.cfg.geometry.y_box_coord_at_row(row);
		    for col in 0..ncols {
			    let x_coord = self.cfg.geometry.x_box_coord_at_col(col);
			    let (mut font_color, mut box_color) = self.get_color(row, col);
			    // Special case if we have lost
			    let char: u8;
			    let delta: u64 = 120 * self.blinksecs as u64;
                if self.status == GameStatus::Lost && row == nrows-1 && self.ticks % delta < delta / 2 {
                    char = self.answer[col as usize];
                    font_color = Color::WHITE;
                    box_color = Color::from_hex(&self.cfg.colors.answer.as_str()).unwrap();
                } else {
                    char = self.grid[row as usize][col as usize];
                }

                let sz = self.cfg.geometry.boxsz;
                d.draw_rectangle(x_coord as i32, y_coord as i32, sz as i32, sz as i32, box_color);
 
                // Draw the letter inside
                if self.grid[row as usize][col as usize] == 0 {
                    continue
                }

                // Draw the letter in the box
                let font_size = 30;
                let x_letter = x_coord + font_size / 2;
                let y_letter = y_coord + font_size;
                let mut charstr = String::new();
                charstr.push(char as char);
                
                d.draw_text(charstr.as_str(), x_letter as i32, y_letter as i32, font_size as i32, font_color);
		    }
	    }

        // Draw the status area
        let y_status = self.height - self.cfg.geometry.statush;
        d.draw_rectangle(0, y_status, self.width, self.cfg.geometry.statush, self.cfg.get_color("status"));

        // Draw status info
        match self.status {
            GameStatus::Won => {
                self.print_status(d, &self.cfg.messages.win);
            },
            GameStatus::Lost => {
                self.print_status(d, &self.cfg.messages.loose);
            },
            _ => {
                if self.col < self.cfg.geometry.ncols {
                    self.print_status(d, &self.cfg.messages.playing);
                } else {
                    self.print_status(d, &self.cfg.messages.enter);
                }
            }
        }
    }

    fn print_status(&self, d: &mut RaylibDrawHandle, message: &Message) {
        let font_size = self.cfg.fonts.message.size;
        let mut message_width = d.measure_text(&message.title, font_size);
        let mut x = (self.width - message_width) / 2;
        let mut y = self.height - self.cfg.geometry.statush + font_size;
        d.draw_text(message.title.as_str(), x, y, font_size, Color::BLACK);

        message_width = d.measure_text(&message.text, font_size);
        x = (self.width - message_width) / 2;
        y += font_size;
        d.draw_text(message.text.as_str(), x, y, font_size, Color::BLACK);
    }
    
}
