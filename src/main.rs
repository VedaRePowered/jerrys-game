use macroquad::{prelude::*, audio::{load_sound, play_sound_once}};

#[macroquad::main("BasicShapes")]
async fn main() {
    let balloon_textures = &[
        load_texture("textures/balloon1.png").await.unwrap(),
        load_texture("textures/balloon2.png").await.unwrap(),
        load_texture("textures/balloon3.png").await.unwrap(),
        load_texture("textures/balloon4.png").await.unwrap(),
        load_texture("textures/balloon5.png").await.unwrap(),
    ];
    let pop_sounds = &[
        load_sound("audio/pop1.wav").await.unwrap(),
        load_sound("audio/pop2.wav").await.unwrap(),
    ];
    let mut balloons = Vec::new();
    let mut timer = 0.0;
    loop {
        clear_background(Color::new(0.0, 0.15, 0.2, 1.0));

        let delta = get_frame_time();
        timer -= delta;
        while timer <= 0.0 {
            balloons.push((
                (rand::gen_range(100.0, screen_width()-100.0), -200.0),
                rand::gen_range(0, balloon_textures.len()),
                rand::gen_range(200.0, 250.0),
            ));
            timer += rand::gen_range(0.0, 1.0);
        }
        for (pos, variant, spd) in &mut balloons {
            pos.1 += *spd * delta;
            let tex = &balloon_textures[*variant];
            draw_texture_ex(
                tex,
                pos.0 - tex.size().x / 4.0,
                screen_height() - pos.1 - tex.size().y / 4.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(tex.size() / 2.0),
                    ..DrawTextureParams::default()
                },
            );
        }

        let touches = touches()
            .into_iter()
            .filter(|t| t.phase == TouchPhase::Started)
            .map(|t| t.position)
            .chain(
                std::iter::once(Vec2::from(mouse_position()))
                    .filter(|_| is_mouse_button_pressed(MouseButton::Left)),
            );
        for touch in touches {
            for i in (0..balloons.len()).rev() {
                if (Vec2::new(balloons[i].0.0, screen_height()-balloons[i].0.1)-touch).length_squared() < 10000.0 {
                    play_sound_once(&pop_sounds[rand::gen_range(0, pop_sounds.len())]);
                    balloons.remove(i);
                    break;
                }
            }
        }

        next_frame().await
    }
}
