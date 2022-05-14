use ::rand::Rng;
use macroquad::prelude::*;
use std::collections::HashMap;
use std::time;

mod effects;
mod player;
mod projectile;
mod entities;
use crate::effects::BackgroundStars;
use crate::player::Player;
use crate::projectile::Bullet;
use entities::Entity;

fn return_conf() -> Conf {
    Conf {
        window_width: 1100,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

fn handle_screen_shake(screen_shake: &mut f32, dt: f32, camera: &mut Vec2, screen_shake_val: i32) {
    *screen_shake -= 1.3 * dt;

    let mut render_offset = Vec2::new(0.0, 0.0);
    if screen_shake > &mut 0.0 {
        render_offset.x = (::rand::thread_rng().gen_range(0..screen_shake_val * 2) - screen_shake_val) as f32;
        render_offset.y = (::rand::thread_rng().gen_range(0..screen_shake_val * 2) - screen_shake_val) as f32;
    }

    *camera += render_offset;
}

#[macroquad::main(return_conf)]
async fn main() {
    let mut player = Player {
        ..Default::default()
    };

    let mut dt = 0.0;
    let mut camera = Vec2::from([0.0, 0.0]);

    let border_radius = 1500.0;
    let border_center = Vec2::from([60.0, 60.0]);

    let mut background_effect = BackgroundStars {
        ..Default::default()
    };

    let mut spaceship_assets: HashMap<&str, Texture2D> = HashMap::new();
    spaceship_assets.insert(
        "basic",
        Texture2D::from_file_with_format(
            include_bytes!("../assets/sprites/space_ships/basic.png"),
            None,
        ),
    );
    let mut screen_shake: f32 = 0.0;
    let mut screen_shake_val = 0; 

    let mut basic_spaceship = Entity {
            movement: Vec2::new(0.0, 0.0),
            vec: Vec2::new(0.0, 0.0),
            angle: 0.0,
            image: spaceship_assets["basic"],
            texture_params: DrawTextureParams {
                dest_size: Option::from(Vec2::new(100.0, 100.0)),
                ..Default::default()
            },
            speed: 3.5
    };

    let mut bullets: Vec<Bullet> = Vec::new();
    let mut event_info: HashMap<&str, f32> = HashMap::new();
    event_info.insert("dt", 0.0);
    event_info.insert("raw dt", 0.0);
    loop {
        // Delta time calculation
        let start = time::Instant::now();
        // Camera smoothness
        camera[0] += (player.vec.x - camera[0] - (screen_width() / 2.0)) * 0.08;
        camera[1] += (player.vec.y - camera[1] - (screen_height() / 2.0)) * 0.08;

        player.update(&mut bullets, &event_info);

        clear_background(BLACK);
        background_effect.draw(camera);
        player.draw(camera);
        draw_circle_lines(
            border_center[0] - camera[0],
            border_center[1] - camera[1],
            border_radius,
            10.0,
            RED,
        );

        // Bullets
        for bullet in &mut bullets {
            bullet.update(dt);
            bullet.draw(camera);

            if bullet.vec.distance(basic_spaceship.vec) < 100.0 {
                screen_shake = 30.0;
                screen_shake_val = 3;
            }
        }
        bullets.retain(|x| x.alive);

        // Screen shake
        if screen_shake > 0.0 {
            handle_screen_shake(&mut screen_shake, event_info["dt"], &mut camera, screen_shake_val);
        }

        // Make space ship move towards player.
        basic_spaceship.update(player.vec, &event_info);
        basic_spaceship.draw(camera);

        // Delta time calculation
        let raw_dt = start.elapsed().as_secs_f32();
        dt = raw_dt * 60.0;

        event_info.insert("dt", dt);
        event_info.insert("raw dt", raw_dt);
        next_frame().await
    }
}
