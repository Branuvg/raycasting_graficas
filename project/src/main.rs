// main.rs
#![allow(unused_imports)]
#![allow(dead_code)]

mod framebuffer;
mod maze;
mod player;
mod caster;
mod textures;
mod enemy;

use raylib::prelude::*;
use std::thread;
use std::time::Duration;
use player::{Player, process_events};
use framebuffer::Framebuffer;
use maze::{Maze,load_maze};
use caster::{cast_ray, Intersect};
use std::f32::consts::PI;
use textures::TextureManager;
use enemy::Enemy;

const TRANSPARENT_COLOR: Color = Color::new(0, 0, 0, 0);

fn draw_sprite(
    framebuffer: &mut Framebuffer,
    player: &Player,
    enemy: &Enemy,
    texture_manager: &TextureManager,
    flashlight_radius: f32,
) {
    let sprite_a = (enemy.pos.y - player.pos.y).atan2(enemy.pos.x - player.pos.x);
    let mut angle_diff = sprite_a - player.a;
    while angle_diff > PI {
        angle_diff -= 2.0 * PI;
    }
    while angle_diff < -PI {
        angle_diff += 2.0 * PI;
    }

    if angle_diff.abs() > player.fov / 2.0 {
        return;
    }

    let sprite_d = ((player.pos.x - enemy.pos.x).powi(2) + (player.pos.y - enemy.pos.y).powi(2)).sqrt();

    // near plane              far plane
    if sprite_d < 50.0 || sprite_d > 300.0 {
        return;
    }

    let screen_height = framebuffer.height as f32;
    let screen_width = framebuffer.width as f32;
    let screen_center_x = screen_width / 2.0;
    let screen_center_y = screen_height / 2.0;

    let sprite_size = (screen_height / sprite_d) * 70.0;
    let screen_x = ((angle_diff / player.fov) + 0.5) * screen_width;

    let start_x = (screen_x - sprite_size / 2.0).max(0.0) as usize;
    let start_y = (screen_height / 2.0 - sprite_size / 2.0).max(0.0) as usize;
    let sprite_size_usize = sprite_size as usize;
    let end_x = (start_x + sprite_size_usize).min(framebuffer.width as usize);
    let end_y = (start_y + sprite_size_usize).min(framebuffer.height as usize);

    for x in start_x..end_x {
        for y in start_y..end_y {
            let tx = ((x - start_x) * 128 / sprite_size_usize) as u32;
            let ty = ((y - start_y) * 128 / sprite_size_usize) as u32;

            let color = texture_manager.get_pixel_color(enemy.texture_key, tx, ty);
            
            if color != TRANSPARENT_COLOR {
                let dist_from_center = ((x as f32 - screen_center_x).powi(2) + (y as f32 - screen_center_y).powi(2)).sqrt();
                
                let flashlight_brightness = if dist_from_center < flashlight_radius {
                    let falloff = 1.0 - (dist_from_center / flashlight_radius);
                    falloff * falloff
                } else {
                    0.0
                };
                
                let distance_fade = (1.0 - (sprite_d / 1000.0)).max(0.0);
                let final_brightness = flashlight_brightness * distance_fade;
                
                let final_color = Color::new(
                    (color.r as f32 * final_brightness) as u8,
                    (color.g as f32 * final_brightness) as u8,
                    (color.b as f32 * final_brightness) as u8,
                    color.a
                );
                
                framebuffer.set_current_color(final_color);
                framebuffer.set_pixel(x as i32, y as i32);
            }
        }
    }
}

fn draw_cell(
    framebuffer: &mut Framebuffer,
    xo: usize,
    yo: usize,
    block_size: usize,
    cell: char,
) {
    if cell == ' ' {
        return;
    }

    framebuffer.set_current_color(Color::RED);

    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            framebuffer.set_pixel(x as i32, y as i32);
        }
    }
}

pub fn render_maze(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    block_size: usize,
    player: &Player,
) {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = col_index * block_size;
            let yo = row_index * block_size;
            
            draw_cell(framebuffer, xo, yo, block_size, cell);
        }
    }
    //draw player
    framebuffer.set_current_color(Color::WHITE);
    let px = player.pos.x as i32;
    let py = player.pos.y as i32;
    framebuffer.set_pixel(px, py);

    let num_rays = 20;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let  a = (player.a - (player.fov / 2.0)) + (player.fov * current_ray);
        cast_ray(framebuffer, &maze, &player, a, block_size, true);
    }
}

pub fn render_3d(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    block_size: usize,
    player: &Player,
    texture_cache: &TextureManager,
    flashlight_radius: f32,
) {
    let num_rays = framebuffer.width;
    let hh = framebuffer.height as f32/ 2.0;
    let screen_width = framebuffer.width as f32;
    let screen_center_x = screen_width / 2.0;
    let screen_center_y = hh;


    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = (player.a - (player.fov / 2.0)) + (player.fov * current_ray);
        let angle_diff = a - player.a;
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);
        let d = intersect.distance;
        let c = intersect.impact;
        let corrected_distance = d * angle_diff.cos() as f32;
        let stake_height = (hh / corrected_distance)*100.0; //factor de escala rendering
        let half_stake_height = stake_height / 2.0;
        let stake_top = (hh - half_stake_height) as usize;
        let stake_bottom = (hh + half_stake_height) as usize;

        for y in stake_top..stake_bottom {
            let tx = intersect.tx;
            let ty = ((y as f32 - stake_top as f32) / (stake_bottom as f32 - stake_top as f32))*128.0; //el 128 tiene que ver con el tamaño de la textura (el ancho), cambiar tanto en main como en caster
            let color = texture_cache.get_pixel_color(c, tx as u32, ty as u32);

            // Flashlight
            let dist_from_center = ((i as f32 - screen_center_x).powi(2) + (y as f32 - screen_center_y).powi(2)).sqrt();

            let flashlight_brightness = if dist_from_center < flashlight_radius {
                let falloff = 1.0 - (dist_from_center / flashlight_radius);
                falloff * falloff
            } else {
                0.0
            };
            
            let distance_fade = (1.0 - (corrected_distance / 1000.0)).max(0.0);
            let final_brightness = flashlight_brightness * distance_fade;
            
            let final_color = Color::new(
                (color.r as f32 * final_brightness) as u8,
                (color.g as f32 * final_brightness) as u8,
                (color.b as f32 * final_brightness) as u8,
                color.a
            );

            framebuffer.set_current_color(final_color);
            framebuffer.set_pixel(i, y as i32);
        }
    }
}

fn render_enemies(
    framebuffer: &mut Framebuffer,
    player: &Player,
    texture_cache: &TextureManager,
    flashlight_radius: f32,
) {
    let enemies = vec![
        Enemy::new(250.0, 250.0, 'e'), //renderización del enemigo
    ];

    for enemy in enemies {
        draw_sprite(framebuffer, &player, &enemy, texture_cache, flashlight_radius);
    }
}

// minimapa
fn render_minimap(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    block_size: usize,
    window_width: i32,
) {
    const MINIMAP_SCALE: f32 = 0.15; // Escala del minimapa
    let map_width = (maze[0].len() as f32 * block_size as f32 * MINIMAP_SCALE) as i32;
    
    // Posición en la esquina superior derecha
    const BORDER_OFFSET: i32 = 10;
    let offset_x = window_width - map_width - BORDER_OFFSET;
    let offset_y = BORDER_OFFSET;

    // Dibujar las paredes del laberinto en el minimapa
    for (j, row) in maze.iter().enumerate() {
        for (i, &cell) in row.iter().enumerate() {
            if cell != ' ' {
                let rect_x = offset_x + (i as f32 * block_size as f32 * MINIMAP_SCALE) as i32;
                let rect_y = offset_y + (j as f32 * block_size as f32 * MINIMAP_SCALE) as i32;
                let rect_w = (block_size as f32 * MINIMAP_SCALE) as i32;
                let rect_h = (block_size as f32 * MINIMAP_SCALE) as i32;

                // Dibujar un rectángulo para cada pared
                framebuffer.set_current_color(Color::new(100, 100, 100, 180)); // Gris semi-transparente
                for y_offset in 0..rect_h {
                    for x_offset in 0..rect_w {
                        framebuffer.set_pixel(rect_x + x_offset, rect_y + y_offset);
                    }
                }
            }
        }
    }

    // Dibujar la posición del jugador en el minimapa
    let player_map_x = offset_x + (player.pos.x * MINIMAP_SCALE) as i32;
    let player_map_y = offset_y + (player.pos.y * MINIMAP_SCALE) as i32;

    framebuffer.set_current_color(Color::WHITE);
    for dy in -2..=2 {
        for dx in -2..=2 {
            framebuffer.set_pixel(player_map_x + dx, player_map_y + dy);
        }
    }

    // Dibujar la dirección del jugador
    let line_length = 15.0; // Longitud de la línea de dirección
    let end_x = player_map_x as f32 + line_length * player.a.cos();
    let end_y = player_map_y as f32 + line_length * player.a.sin();

    // Dibujar una línea simple
    for i in 0..15 {
        let t = i as f32 / 14.0;
        let x = player_map_x as f32 * (1.0 - t) + end_x * t;
        let y = player_map_y as f32 * (1.0 - t) + end_y * t;
        framebuffer.set_pixel(x as i32, y as i32);
    }
}


fn main() {
    let window_width = 1300;
    let window_height = 900;
    let block_size = 100;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raycaster")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    window.disable_cursor();

    let mut framebuffer = Framebuffer::new(
        window_width as i32, 
        window_height as i32, 
        Color::BLACK
    );

    framebuffer.set_background_color(Color::BLACK);

    // Load the maze once before the loop
    let maze = load_maze("maze.txt");
    let mut player = Player{
        pos: Vector2::new(150.0,150.0), 
        a: PI/2.0,
        fov: PI / 3.0, 
    };

    let texture_cache = TextureManager::new(&mut window, &raylib_thread);
    
    let flashlight_radius = 500.0; //Radio del haz de luz

    while !window.window_should_close() {
        // 1. clear framebuffer
        framebuffer.clear();
        
        let screen_center_x = (window_width / 2) as f32;
        let screen_center_y = (window_height / 2) as f32;
        let half_height = (window_height / 2) as i32;

        let floor_color = Color::new(51, 25, 0, 255); // Un café oscuro del piso

        for y in half_height..window_height as i32 {
            for x in 0..window_width as i32 {
                let dist_from_center = ((x as f32 - screen_center_x).powi(2) + (y as f32 - screen_center_y).powi(2)).sqrt();
                
                let brightness = if dist_from_center < flashlight_radius {
                    let falloff = 1.0 - (dist_from_center / flashlight_radius);
                    // Hacemos que la atenuación sea menos pronunciada para el suelo
                    falloff
                } else {
                    0.0
                };
                
                let final_color = Color::new(
                    (floor_color.r as f32 * brightness) as u8,
                    (floor_color.g as f32 * brightness) as u8,
                    (floor_color.b as f32 * brightness) as u8,
                    255
                );

                framebuffer.set_current_color(final_color);
                framebuffer.set_pixel(x, y);
            }
        }

        let mouse_delta_x = window.get_mouse_delta().x; // Obtener el movimiento horizontal del mouse

        // 1.1 process events
        process_events(&window, &mut player, &maze, block_size, mouse_delta_x);

        // 2. draw the maze, passing the maze and block size
        let mut mode = "3D";
        
        if window.is_key_down(KeyboardKey::KEY_M) {
            mode = "2D";
        }

        if mode == "2D" {
            render_maze(&mut framebuffer, &maze, block_size, &player);
        } else {
            render_3d(&mut framebuffer, &maze, block_size, &player, &texture_cache, flashlight_radius);
            render_enemies(&mut framebuffer, &player, &texture_cache, flashlight_radius);
        }

        // --- MINIMAPA --- Llamar a la función de renderizado del minimapa
        if mode != "2D" {
            render_minimap(&mut framebuffer, &maze, &player, block_size, window_width);
        }

        // 3. swap buffers
        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}
