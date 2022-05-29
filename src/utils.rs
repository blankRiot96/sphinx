use ::rand::Rng;
use macroquad::prelude::*;
use std::time::{self, Instant};

pub fn handle_screen_shake(screen_shake: &mut f32, dt: f32, camera: &mut Vec2, screen_shake_val: i32) {
    *screen_shake -= 1.3 * dt;

    let mut render_offset = Vec2::new(0.0, 0.0);
    if screen_shake > &mut 0.0 {
        render_offset.x =
            (::rand::thread_rng().gen_range(0..screen_shake_val * 2) - screen_shake_val) as f32;
        render_offset.y =
            (::rand::thread_rng().gen_range(0..screen_shake_val * 2) - screen_shake_val) as f32;
    }

    *camera += render_offset;
}


pub struct Time {
    pub time_to_pass: f32,
    pub start: Instant
}

impl Default for Time {
    fn default() -> Self {
        let start = time::Instant::now();
        Time {
            time_to_pass: 0.0,
            start
        }
    }
} 

impl Time {
    pub fn update(&mut self) -> bool {
        if self.start.elapsed().as_secs_f32() > self.time_to_pass {
            self.start = time::Instant::now();
            return true;
        }
        false
    }
}
