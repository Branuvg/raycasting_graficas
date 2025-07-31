use raylib::prelude::*;
use crate::player::Player;
use crate::maze::Maze;
use crate::framebuffer::Framebuffer;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    a: f32,
    block_size: usize,
    draw: bool,
) -> Intersect {
    let mut d = 0.0;
    framebuffer.set_current_color(Color::WHITE);

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();
        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;

        let i = x / block_size;
        let j = y / block_size;
        
        if maze[j][i] != ' ' {
            return Intersect {
                distance: d,
                impact: maze[j][i],
            }; 
        }
        
        if draw {
            framebuffer.set_pixel(x as i32, y as i32);
        }
        
        d += 1.0;
    }
}