//framebuffer.rs
use raylib::prelude::*;

pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    pub color_buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width, height, background_color);
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        self.color_buffer.clear_background(self.background_color);
    }

    pub fn set_pixel(&mut self, x: i32, y: i32) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.color_buffer.draw_pixel(x, y, self.current_color);
        }
    }
    
    pub fn get_pixel_color(&mut self, x: i32, y: i32) -> Option<Color> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Some(self.color_buffer.get_color(x, y))
        } else {
            None
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    // --- CAMBIO --- Eliminada la lógica de dibujado de FPS
    pub fn swap_buffers(&self, window: &mut RaylibHandle, raylib_thread: &RaylibThread) -> Option<Texture2D> {
        window.load_texture_from_image(raylib_thread, &self.color_buffer).ok()
    }
}