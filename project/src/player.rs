//player.rs
use raylib::prelude::*;
use std::f32::consts::PI;
use crate::maze::Maze;

pub struct Player {
    pub pos: Vector2,
    pub a: f32,
    pub fov: f32,
}

pub fn process_events( //Comprobar si el jugador ha llegado a la meta
    window: &RaylibHandle,
    player: &mut Player,
    maze: &Maze,
    block_size: usize,
    mouse_delta_x: f32,
    goal_unlocked: bool,
) -> bool {
    const MOVE_SPEED: f32 = 8.0;
    const ROTATION_SPEED: f32 = PI / 40.0;
    const MOUSE_SENSITIVITY: f32 = 0.01;

    //Rotación
    if window.is_key_down(KeyboardKey::KEY_LEFT) || window.is_key_down(KeyboardKey::KEY_A) {
        player.a -= ROTATION_SPEED;
    }
    if window.is_key_down(KeyboardKey::KEY_RIGHT) || window.is_key_down(KeyboardKey::KEY_D) {
        player.a += ROTATION_SPEED;
    }

    player.a += mouse_delta_x * MOUSE_SENSITIVITY;

    let mut next_pos = player.pos;
    let mut moved = false;

    //Movimiento
    if window.is_key_down(KeyboardKey::KEY_UP) || window.is_key_down(KeyboardKey::KEY_W) || window.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
        next_pos.x += MOVE_SPEED * player.a.cos();
        next_pos.y += MOVE_SPEED * player.a.sin();
        moved = true;
    }
    if window.is_key_down(KeyboardKey::KEY_DOWN) || window.is_key_down(KeyboardKey::KEY_S) || window.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
        next_pos.x -= MOVE_SPEED * player.a.cos();
        next_pos.y -= MOVE_SPEED * player.a.sin();
        moved = true;
    }

    if moved {
        let grid_x = next_pos.x as usize / block_size;
        let grid_y = next_pos.y as usize / block_size;

        if grid_y < maze.len() && grid_x < maze[grid_y].len() {
            if goal_unlocked && maze[grid_y][grid_x] == 'g' { //Comprobar si el jugador ha llegado a la meta
                return true;
            }

            if maze[grid_y][grid_x] == ' ' {
                player.pos = next_pos;
            }
        }
    }
    
    false // No se ha ganado
}