use macroquad::prelude::*;
use std::time;

#[allow(unused_imports)]
use crate::effects::BackgroundStars;
use crate::player::Player;
mod effects;
mod player;

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
    loop {
        // Delta time calculation
        let start = time::Instant::now();

        player.update(dt);

        clear_background(BLACK);
        background_effect.draw(camera);
        player.draw(camera);
        draw_rectangle(50.0 - camera[0], 50.0 - camera[1], 60.0, 75.0, RED);
        draw_circle_lines(
            border_center[0] - camera[0],
            border_center[1] - camera[1],
            border_radius,
            10.0,
            RED,
        );

        // Camera smoothness
        camera[0] += (player.vec.x - camera[0] - (screen_width() / 2.0)) * 0.03;
        camera[1] += (player.vec.y - camera[1] - (screen_height() / 2.0)) * 0.03;

        // Delta time calculation
        let raw_dt = start.elapsed().as_secs_f32();
        dt = raw_dt * 60.0;
        next_frame().await
    }
}
