use lazy_static::lazy_static;
use macroquad::prelude::*;

pub fn get_movement(pos: Vec2, target: Vec2, speed: f32) -> [f32; 3] {
    let angle = f32::atan2(target[1] - pos[1], target[0] - pos[0]);
    let dx = angle.cos() * speed;
    let dy = angle.sin() * speed;

    [dx, dy, angle]
}

pub struct Bullet {
    pub distance: f32,
    pub damage: i32,
    pub vec: Vec2,
    pub movement: Vec2,
    pub color: Color,
    pub texture_params: DrawTextureParams,
    pub alive: bool,

    pub distance_travelled: f32,
}

impl Default for Bullet {
    fn default() -> Self {
        let default_coordinate = Vec2::new(0.0, 0.0);
        Bullet {
            distance: 0.0,
            damage: 0,
            vec: default_coordinate,
            movement: default_coordinate,
            texture_params: DrawTextureParams {
                ..Default::default()
            },
            distance_travelled: 0.0,
            alive: true,
            color: WHITE,
        }
    }
}

lazy_static! {
    static ref IMAGE: Texture2D =
        Texture2D::from_file_with_format(include_bytes!("../assets/sprites/bullet.png"), None);
}

impl Bullet {
    pub fn update(&mut self, dt: f32) {
        if self.distance_travelled < self.distance {
            self.vec.x += self.movement.x * dt;
            self.vec.y += self.movement.y * dt;

            self.distance_travelled +=
                ((self.movement.x * dt).powi(2) + (self.movement.y * dt).powi(2)).sqrt();
        } else {
            self.alive = false;
        }
    }

    pub fn draw(&self, camera: Vec2) {
        draw_texture_ex(
            *IMAGE,
            self.vec.x - camera.x,
            self.vec.y - camera.y,
            self.color,
            self.texture_params.clone(),
        );
    }
}
