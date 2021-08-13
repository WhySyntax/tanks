use crate::objects::{Bullets, Obstacles};
use coffee::graphics::{Frame, Mesh, Rectangle, Color, Shape};

const SCREEN_WIDTH:u32 = 1280;
const SCREEN_HEIGHT:u32 = 1024;
const CELL_DIMENSIONS:[u32;2] = [81,64];

#[derive(Copy, Clone)]
pub(crate) struct PlayerTank {
    pub(crate) sublocation: [u8;2],
    pub(crate) location: [i32;2]
}
impl PlayerTank {
    pub(crate) fn draw(&mut self, frame: &mut Frame) {
        let mut mesh = Mesh::new();
        mesh.fill(
            Shape::Rectangle(Rectangle {
                x: self.location[0] as f32,
                y: self.location[1] as f32,
                width: CELL_DIMENSIONS[0] as f32 * 7.0_f32 / 8.0_f32,
                height: CELL_DIMENSIONS[1] as f32 * 7.0_f32 / 8.0_f32,
            }),
            Color::GREEN,
        );
        mesh.draw(&mut frame.as_target());
    }
}

#[derive(Copy, Clone)]
pub(crate) struct EnemyTank {
    pub(crate) sublocation: [u8;2],
    pub(crate) location: [i32;2],
    pub(crate) speed_shooter: bool,
    pub(crate) ricochets: bool,
    pub(crate) turret: bool
}

impl EnemyTank {
    pub(crate) fn calculate_shot(&self, player:PlayerTank, map:[[Option<Obstacles>;16];16]) -> Option<Bullets> {
        //just the ratio
        let ratio:f32 = ((player.sublocation[1] as i8 - self.sublocation[1] as i8) / (player.sublocation[0] as i8 - self.sublocation[0] as i8)) as f32;
        //check for obstacles
        // for i in 0..16 {
        //     if ((i<player.sublocation[0])&&(i<self.sublocation[0]))||((i>player.sublocation[0])&&(i>self.sublocation[0])) {
        //         continue;
        //     }
        //     //map[i as usize][(ratio*i as f32).ceil() as usize]
        //     match map[i as usize][(ratio*i as f32).floor() as usize] {
        //         Some(T) => {if T.bullet_collision {return self.calculate_bank(player,map);}},
        //         _ => {
        //             match map[i as usize][(ratio*i as f32).ceil() as usize] {
        //                 Some(T) => {if T.bullet_collision {return self.calculate_bank(player,map);}},
        //                 _ => continue
        //             }
        //         }
        //     };
        // }
        //if nothing's in the way calculate the shot
        let heading:f32 = ratio.powi(-1);
        let mut speed = 10;
        if self.sublocation>player.sublocation {
            speed = -10;
        }

        Some(Bullets {
            heading,
            speed,
            location: self.location,
            sublocation: self.sublocation
        })
    }

    fn calculate_bank(&self, player:PlayerTank, map:[[Option<Obstacles>;16];16]) -> Option<Bullets> {
        if !self.ricochets {
            return None
        }
        None
    }

    pub(crate) fn draw(&mut self, frame: &mut Frame) {
        let mut mesh = Mesh::new();
        mesh.fill(
            Shape::Rectangle(Rectangle {
                x: self.location[0] as f32,
                y: self.location[1] as f32,
                width: CELL_DIMENSIONS[0] as f32 * 7.0_f32 / 8.0_f32,
                height: CELL_DIMENSIONS[1] as f32 * 7.0_f32 / 8.0_f32,
            }),
            Color::RED,
        );
        mesh.draw(&mut frame.as_target());
    }
}

fn calculate_location(sublocation:[u8;2]) -> [i32;2] {
    [sublocation[0] as i32 *SCREEN_WIDTH as i32 /16,sublocation[1] as i32 *SCREEN_HEIGHT as i32 /16]
}
fn calculate_sublocation(location:[i32;2]) -> [u8;2] {
    [(location[0]*16/SCREEN_WIDTH as i32) as u8,(location[1]*16/SCREEN_HEIGHT as i32) as u8]
}