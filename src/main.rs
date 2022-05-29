use macroquad::prelude::*;
use std::collections::HashMap;
use std::time;

mod effects;
mod entities;
mod player;
mod projectile;
mod utils;
use crate::effects::BackgroundStars;
use crate::player::Player;
use crate::projectile::Bullet;
use entities::{Entity, Entities};
use utils::{handle_screen_shake, Time};


fn return_conf() -> Conf {
    Conf {
        window_width: 1100,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
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

    let mut screen_shake: f32 = 0.0;
    let mut screen_shake_val = 0;


    let mut bullets: Vec<Bullet> = Vec::new();
    let mut entities: Vec<Entity> = Vec::new();
    let mut entity_spawn_time: Time = Time {
        time_to_pass: 1.4,
        ..Default::default()
    };

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

            for entity in &mut entities {
                if bullet.vec.distance(entity.vec) < 100.0 {
                    screen_shake = 30.0;
                    screen_shake_val = 3;
                    entity.hp -= bullet.damage;
                    bullet.alive = false;
                }
            }
        }
        bullets.retain(|x| x.alive);

        // Spawn enemies
        if entity_spawn_time.update() {
            entities.push(Entity::new(Entities::SpeedyEnemy));
        }

        // Screen shake
        if screen_shake > 0.0 {
            handle_screen_shake(
                &mut screen_shake,
                event_info["dt"],
                &mut camera,
                screen_shake_val,
            );
        }

        for entity in &mut entities {
            entity.update(player.vec, &event_info);
            entity.draw(camera);
        }
        entities.retain(|x| x.hp > 0);

        // Delta time calculation
        let raw_dt = start.elapsed().as_secs_f32();
        dt = raw_dt * 60.0;

        event_info.insert("dt", dt);
        event_info.insert("raw dt", raw_dt);
        next_frame().await
    }
}
