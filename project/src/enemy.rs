// enemy.rs
use raylib::prelude::*;

pub struct Enemy {
    pub pos: Vector2,
    pub texture_key: char,
    animation_timer: f32,
}

impl Enemy {
    pub fn new(x: f32, y: f32) -> Self {
        Enemy {
            pos: Vector2::new(x, y), 
            texture_key: 'e', // Empezar con el primer frame
            animation_timer: 0.0,
        }
    }

    // Este método ahora solo se encarga de la animación del sprite
    pub fn update(&mut self, delta_time: f32) {
        // --- Lógica de Animación ---
        // Alternar entre 'e' (enemy1.png) y 'f' (enemy2.png)
        self.animation_timer += delta_time;
        if self.animation_timer > 0.4 { // Cambiar de frame cada 0.4 segundos
            self.animation_timer = 0.0;
            if self.texture_key == 'e' {
                self.texture_key = 'f';
            } else {
                self.texture_key = 'e';
            }
        }
    }
}