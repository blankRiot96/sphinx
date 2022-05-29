use crate::projectile;
use macroquad::prelude::*;
use std::collections::HashMap;


pub enum Entities {
    BasicEnemy,
    SpeedyEnemy
}

pub struct Entity {
    pub movement: Vec2,
    pub vec: Vec2,
    pub angle: f32,
    pub image: Texture2D,
    pub texture_params: DrawTextureParams,
    pub speed: f32,
    pub hp: i32,
    pub kind: Entities
}

impl Entity {
    pub fn new(entity_type: Entities) -> Self {

    match entity_type {
        Entities::BasicEnemy => {
            let image = Texture2D::from_file_with_format(
                include_bytes!("../assets/sprites/space_ships/basic.png"),
                None,
            );
            Entity {
                    movement: Vec2::new(0.0, 0.0),
                    vec: Vec2::new(0.0, 0.0),
                    angle: 0.0,
                    image,
                    texture_params: DrawTextureParams {
                        dest_size: Option::from(Vec2::new(100.0, 100.0)),
                        ..Default::default()
                    },
                    speed: 3.5,
                    hp: 100,
                    kind: Entities::BasicEnemy
                }
            },
        Entities::SpeedyEnemy => {
            let image = Texture2D::from_file_with_format(
                include_bytes!("../assets/sprites/space_ships/speedy.png"),
                None,
            );
            Entity {
                    movement: Vec2::new(0.0, 0.0),
                    vec: Vec2::new(0.0, 0.0),
                    angle: 0.0,
                    image,
                    texture_params: DrawTextureParams {
                        dest_size: Option::from(Vec2::new(70.0, 70.0)),
                        ..Default::default()
                    },
                    speed: 7.5,
                    hp: 30,
                    kind: Entities::BasicEnemy
                }
            },
        }
    }

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
        self.vec += self.movement * Vec2::splat(event_info["dt"]);
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

