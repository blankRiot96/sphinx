use macroquad::prelude::*;

mod projectile {
    include!("projectile.rs");
}
use crate::player::projectile::get_movement;

pub struct Player {
    pub vec: Vec2,
    pub speed: f32,
    pub image: Texture2D,
    pub turret_image: Texture2D,
    pub texture_params: DrawTextureParams,
    pub turret_params: DrawTextureParams,
    pub last_rotation_target: f32,
    pub camera: Vec2,
}

impl Default for Player {
    fn default() -> Self {
        let image = Texture2D::from_file_with_format(
            include_bytes!("../assets/sprites/space_ships/01/Spaceship_01_BLUE.png"),
            None,
        );
        let turret_image =
            Texture2D::from_file_with_format(include_bytes!("../assets/sprites/turret.png"), None);
        let text_params = DrawTextureParams {
            dest_size: Option::from(Vec2::new(100.0, 100.0)),
            flip_x: false,
            flip_y: false,
            pivot: None,
            rotation: 0.0,
            source: None,
        };

        let turret_params = DrawTextureParams {
            dest_size: Some(Vec2::new(25.0, 25.0)),
            flip_x: false,
            flip_y: false,
            pivot: None,
            rotation: 0.0,
            source: None,
        };
        Player {
            vec: Vec2::new(100.0, 100.0),
            speed: 5.5,
            image: image,
            turret_image: turret_image,
            texture_params: text_params,
            turret_params: turret_params,
            last_rotation_target: 0.0,
            camera: Vec2::new(0.0, 0.0),
        }
    }
}

impl Player {
    fn handle_turret(&mut self) {
        self.turret_params.rotation = Vec2::new(self.vec.x - self.camera.x, 
                            self.vec.y - self.camera.y)
        .angle_between(Vec2::from(mouse_position())) + 1.6;

        if is_mouse_button_pressed(MouseButton::Left) {
            println!("Yes?");
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Check desired rotation based on input
        let mut target_rotation: f32 = self.last_rotation_target;
        let (mut dx, mut dy) = (0.0, 0.0);
        if is_key_down(KeyCode::D) {
            dx += self.speed * dt;
            target_rotation = 1.6;
        }
        if is_key_down(KeyCode::A) {
            dx -= self.speed * dt;
            target_rotation = -1.6;
        }
        if is_key_down(KeyCode::S) {
            dy += self.speed * dt;
            target_rotation = 3.2;
        }
        if is_key_down(KeyCode::W) {
            dy -= self.speed * dt;
            target_rotation = 0.0;
        }
        if target_rotation != self.last_rotation_target {
            self.last_rotation_target = target_rotation;
        }

        // Handle speed boost
        if is_key_down(KeyCode::LeftShift) {
            self.speed = 8.5;
        } else {
            self.speed = 5.5;
        }

        self.texture_params.rotation = target_rotation;

        if dx != 0.0 && dy != 0.0 {
            if dy < 0.0 {
                if dx > 0.0 {
                    self.texture_params.rotation += 0.8;
                } else {
                    self.texture_params.rotation -= 0.8;
                }
            } else {
                if dx > 0.0 {
                    self.texture_params.rotation -= 0.8;
                } else {
                    self.texture_params.rotation += 0.8;
                }
            }
            dx *= 1.414 / 2.0;
            dy *= 1.414 / 2.0;
        }

        // Make sure Space Ship isn't trying to leave the border
        let adjust_x = if self.vec.x > 60.0 {
            self.texture_params.dest_size.unwrap()[0]
        } else {
            0.0
        };

        let adjust_y = if self.vec.y > 60.0 {
            self.texture_params.dest_size.unwrap()[1]
        } else {
            0.0
        };
        let distance_from_center = ((self.vec.x + dx + adjust_x - 60.0).powi(2)
            + (self.vec.y + dy + adjust_y - 60.0).powi(2))
        .sqrt();
        if distance_from_center > 1500.0 {
            dx = 0.0;
            dy = 0.0;
        }
        self.handle_turret();

        self.vec.x += dx;
        self.vec.y += dy;
    }

    pub fn draw(&mut self, camera: Vec2) {
        // draw_rectangle(self.x, self.y, 60.0, 60.0, RED);
        self.camera = camera;
        draw_texture_ex(
            self.image,
            self.vec.x - camera.x,
            self.vec.y - camera.y,
            WHITE,
            self.texture_params.clone(),
        );
        draw_texture_ex(
            self.turret_image,
            self.vec.x - camera.x + 35.0,
            self.vec.y - camera.y + 35.0,
            WHITE,
            self.turret_params.clone(),
        );
    }
}
