use macroquad::prelude::*;
use lazy_static::lazy_static;


pub fn get_movement(pos: [f32; 2], target: [f32; 2], speed: f32) -> [f32; 3] {
    let angle = f32::atan2(target[1] - pos[1], target[0] - pos[0]);
    let dx = angle.cos() * speed;
    let dy = angle.sin() * speed;

    [dx, dy, angle]
}

pub struct Bullet {
    size: f32,
    damage: f32,
    vec: Vec2,
    target: Vec2,
    color: Color,
    texture_params: DrawTextureParams,
    alive: bool
}

lazy_static! {
    static ref IMAGE: Texture2D = 
            Texture2D::from_file_with_format(include_bytes!("../assets/sprites/bullet.png"), None);
}

impl Bullet {
    fn update(&self) {
    }

    fn draw(&self, camera: Vec2) {
        draw_texture_ex(
            *IMAGE,
            self.vec.x - camera.x,
            self.vec.y - camera.y,
            self.color,
            self.texture_params.clone()
        );
    }
}

