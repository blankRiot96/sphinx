use macroquad::prelude::*;
use crate::projectile;
use std::collections::HashMap;

pub struct Entity {
	pub movement: Vec2,
	pub vec: Vec2,
	pub angle: f32,
	pub image: Texture2D,
	pub texture_params: DrawTextureParams,
	pub speed: f32,
}

impl Entity {
	pub fn move_towards(&mut self, target: Vec2) {
		let values = projectile::get_movement(self.vec, target, self.speed);
		self.movement = Vec2::new(values[0], values[1]);
		self.angle = values[2];
	}

	/// Update Entity.
	pub fn update(&mut self, target: Vec2, event_info: &HashMap<&str, f32>) {
		self.move_towards(target);
		self.vec += self.movement * Vec2::new(event_info["dt"], event_info["dt"]);
		self.texture_params.rotation = self.angle + 1.5;
	}

	pub fn draw(&self, camera: Vec2) {
        draw_texture_ex(
            self.image,
            self.vec.x - camera.x,
            self.vec.y - camera.y,
            WHITE,
            self.texture_params.clone(),
        );
	}
}



// pub struct Enemy {

// }

// pub struct BasicEnemy {

// }

