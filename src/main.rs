mod tanks;
mod objects;

extern crate coffee;

use coffee::graphics::{Color, Frame, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Result, Timer};
use std::fs::read_to_string;
use crate::objects::{Bullets,Obstacles};
use crate::tanks::{PlayerTank,EnemyTank};
use coffee::input::KeyboardAndMouse;
use coffee::ui::ProgressBar;

const SCREEN_WIDTH:u32 = 1280;
const SCREEN_HEIGHT:u32 = 1024;
const CELL_DIMENSIONS:[u32;2] = [81,64];
fn main() -> Result<()> {
    KaizoTanks::run(WindowSettings {
        title: String::from("Tanks, like the Wii game, but harder"),
        size: (SCREEN_WIDTH, SCREEN_HEIGHT),
        resizable: false,
        fullscreen: false,
        maximized: false,
    })
}

struct KaizoTanks {
    // game assets? must looks into
    player: PlayerTank,
    enemies: Vec<EnemyTank>,
    map: [[Option<Obstacles>;16];16],
    bullets: Vec<Bullets>,
    ticks: u8
}

impl Game for KaizoTanks {
    type Input = KeyboardAndMouse;
    type LoadingScreen = ();
    const TICKS_PER_SECOND: u16 = 30; // No loading screen

    fn load(_window: &Window) -> Task<KaizoTanks> {
        //pick map to use
        let raw = read_to_string("map_1.txt").expect("invalid file");
        let raw_iter = raw.split(';');
        let raw_vector:Vec<&str> = raw_iter.collect();
        let mut map_vec:Vec<Vec<&str>> = vec![];
        for i in 0..raw_vector.len() {
            let mini_raw_iter = raw_vector[i].trim().split(',');
            let mini_raw_vector:Vec<&str> = mini_raw_iter.collect();
            map_vec.push(mini_raw_vector);
        }
        //use map to initialize player, enemies, and also walls
        let mut player_tank:PlayerTank = PlayerTank {
            sublocation: [0,0],
            location: calculate_location([0,0])
        };
        let mut the_enemies:Vec<EnemyTank> = vec![];
        let mut game_map:[[Option<Obstacles>;16];16] = [[None;16];16];
        for i in 0..map_vec.len() {
            for j in 0..map_vec[i].len() {
                if map_vec[i][j]=="p" {
                    player_tank.sublocation=[i as u8,j as u8];
                    player_tank.location=calculate_location([i as u8,j as u8]);
                } else if map_vec[i][j]=="w" {
                    game_map[i][j]= Option::from(Obstacles {
                        location: [i as u8, j as u8],
                        bullet_collision: true
                    });
                } else if map_vec[i][j]=="h" {
                    game_map[i][j]= Option::from(Obstacles {
                        location: [i as u8, j as u8],
                        bullet_collision: false
                    });
                } else if map_vec[i][j]=="e" {
                    the_enemies.push(EnemyTank {
                        sublocation: [i as u8, j as u8],
                        location: calculate_location([i as u8, j as u8]),
                        speed_shooter: false,
                        ricochets: false,
                        turret: false
                    });
                } else if map_vec[i][j]=="se" {
                    the_enemies.push(EnemyTank {
                        sublocation: [i as u8, j as u8],
                        location: calculate_location([i as u8, j as u8]),
                        speed_shooter: false,
                        ricochets: true,
                        turret: false
                    });
                } else if map_vec[i][j]=="t" {
                    the_enemies.push(EnemyTank {
                        sublocation: [i as u8, j as u8],
                        location: calculate_location([i as u8, j as u8]),
                        speed_shooter: false,
                        ricochets: true,
                        turret: true
                    });
                } else if map_vec[i][j]=="st" {
                    the_enemies.push(EnemyTank {
                        sublocation: [i as u8, j as u8],
                        location: calculate_location([i as u8, j as u8]),
                        speed_shooter: true,
                        ricochets: true,
                        turret: true
                    });
                }
            }
        }
        // Load your game assets here. Check out the `load` module!
        Task::succeed(move || KaizoTanks {
            player: player_tank,
            enemies: the_enemies,
            map: game_map,
            bullets: vec![],
            ticks: 0
        })
    }

    fn update(&mut self, _window: &Window) {
        self.ticks+=1;
        if self.ticks%15==0 {
            for i in 0..self.bullets.len() {
                self.bullets[i].move_forwards();
            }
            if self.ticks%240==0 {
                for i in 0..self.enemies.len() {
                    match self.enemies[i].calculate_shot(self.player, self.map) {
                        Some(T) => self.bullets.push(T),
                        None => continue
                    };
                }
                self.ticks=0;
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        // Clear the current frame
        frame.clear(Color::WHITE);
        // Draw your game here. Check out the `graphics` module!
        self.player.draw(frame);
        //loop to draw walls
        for i in 0..self.map.len() {
            for j in 0..self.map[i].len() {
                match self.map[i][j] {
                    Some(mut T) => T.draw(frame),
                    None => continue
                }
            }
        }
        //loop to draw enemies
        for i in 0..self.enemies.len() {
            self.enemies[i].draw(frame);
        }
        //loop to draw bullets
        for i in 0..self.bullets.len() {
            self.bullets[i].draw(frame);
        }
    }

    fn interact(&mut self, input: &mut KeyboardAndMouse, _window: &mut Window) {

    }
}

// fn read_map_to_array<'a>(file_name:&str) -> Vec<Vec<&'a str>> {
//     let raw = read_to_string(file_name).expect("invalid file");
//     let raw_iter = raw.clone().split(';');
//     let raw_vector:Vec<&str> = raw_iter.collect();
//     let mut the_map:Vec<Vec<&str>> = vec![];
//     for i in 0..raw_vector.len() {
//         let mini_raw_iter = raw_vector[i].trim().split(',');
//         let mini_raw_vector:Vec<&str> = mini_raw_iter.collect();
//         the_map.push(mini_raw_vector);
//     }
//     return the_map
// }

fn calculate_location(sublocation:[u8;2]) -> [i32;2] {
    [sublocation[0] as i32 *SCREEN_WIDTH as i32 /16,sublocation[1] as i32 *SCREEN_HEIGHT as i32 /16]
}
fn calculate_sublocation(location:[i32;2]) -> [u8;2] {
    [(location[0]*16/SCREEN_WIDTH as i32) as u8,(location[1]*16/SCREEN_HEIGHT as i32) as u8]
}