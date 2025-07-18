use raylib::prelude::*;
use std::f32::consts::PI;

pub struct Player {
    pub pos: Vector2,
}

pub fn process_events(window: &RaylibHandle, player: &mut Player) {
    const MOVE_SPEED: f32 = 5.0;

    if window.is_key_pressed(KeyboardKey::KEY_LEFT) {
        player.pos.x -= MOVE_SPEED;
    }
    if window.is_key_pressed(KeyboardKey::KEY_RIGHT) {
        player.pos.x += MOVE_SPEED;
    }
    if window.is_key_pressed(KeyboardKey::KEY_UP) {
        player.pos.y -= MOVE_SPEED;
    }
    if window.is_key_pressed(KeyboardKey::KEY_DOWN) {
        player.pos.y += MOVE_SPEED;
    }
}