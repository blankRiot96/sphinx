use self::rand;
use macroquad::prelude::*;

pub struct Star {
    pub image: Option<Texture2D>,
    texture_params: DrawTextureParams,
    pub vec: Vec2,
}

impl Default for Star {
    fn default() -> Self {
        let size = rand::gen_range(15.0, 25.0);

        Star {
            image: None,
            texture_params: DrawTextureParams {
                dest_size: Option::from(Vec2::new(size, size)),
                flip_x: false,
                flip_y: false,
                pivot: None,
                rotation: 0.0,
                source: None,
            },
            vec: Vec2::new(0.0, 0.0),
        }
    }
}

impl Star {
    fn draw(&mut self, camera: Vec2) {
        self.texture_params.rotation += 0.01;
        self.texture_params.rotation %= 6.2;
        draw_texture_ex(
            self.image.unwrap(),
            self.vec.x - camera[0],
            self.vec.y - camera[1],
            WHITE,
            self.texture_params.clone(),
        )
    }
}

#[allow(dead_code)]
pub struct BackgroundStars {
    pub stars: Vec<Star>,
}

impl Default for BackgroundStars {
    fn default() -> Self {
        let mut stars: Vec<Star> = Vec::new();
        let star_img =
            Texture2D::from_file_with_format(include_bytes!("../assets/sprites/star.png"), None);
        for _ in 0..500 {
            let x = rand::gen_range(60.0 - 1500.0, 60.0 + 1500.0);
            let y = rand::gen_range(60.0 - 1500.0, 60.0 + 1500.0);

            stars.push(Star {
                image: Some(star_img),
                vec: Vec2::new(x, y),
                ..Default::default()
            });
        }
        BackgroundStars { stars }
    }
}

impl BackgroundStars {
    pub fn draw(&mut self, camera: Vec2) {
        for star in &mut self.stars {
            star.draw(camera);
        }
    }
}
