use macroquad::{
    audio::{load_sound, play_sound_once},
    prelude::*,
};

fn center_text(text: String, font: &Font, x: f32, y: f32, height: u16) {
    let size = measure_text(&text, Some(&font), height, 1.0);
    draw_text_ex(
        &text,
        x - size.width / 2.0,
        y,
        TextParams {
            font: Some(&font),
            font_size: height,
            ..TextParams::default()
        },
    );
}

#[macroquad::main("Jerry's Game")]
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
    let score_img = load_texture("textures/score.png").await.unwrap();
    let health_img = load_texture("textures/health.png").await.unwrap();
    let retry_img = load_texture("textures/retry.png").await.unwrap();
    let font = load_ttf_font("single_day.ttf").await.unwrap();
    let poof_textures = &[
        load_texture("textures/poof1.png").await.unwrap(),
        load_texture("textures/poof2.png").await.unwrap(),
        load_texture("textures/poof3.png").await.unwrap(),
        load_texture("textures/poof4.png").await.unwrap(),
    ];
    let mut balloons = Vec::new();
    let mut timer = 0.0;
    let mut health = 10.0;
    let mut score = 0.0;
    let mut combo: f32 = 0.0;
    let mut game_speed = 0.5;
    let mut dead = false;
    let mut health_flash = 0.0;
    let mut poofs = Vec::new();
    //let mut combo_text = Vec::new();
    loop {
        clear_background(Color::new(0.0, 0.15, 0.2, 1.0));

        let touches = touches()
            .into_iter()
            .filter(|t| t.phase == TouchPhase::Started)
            .map(|t| t.position)
            .chain(
                std::iter::once(Vec2::from(mouse_position()))
                    .filter(|_| is_mouse_button_pressed(MouseButton::Left)),
            );

        if dead {
            center_text(
                "You Died.".to_string(),
                &font,
                screen_width() / 2.0,
                screen_height() / 2.0 - 100.0,
                100,
            );
            center_text(
                format!("Congratulations, Jerry, you got a score of {score}."),
                &font,
                screen_width() / 2.0,
                screen_height() / 2.0 - 40.0,
                40,
            );
            let retry_pos = Vec2::new(screen_width() / 2.0 - retry_img.size().x / 8.0,
                screen_height() / 2.0 + 20.0);
            let retry_size = retry_img.size() / 4.0;
            draw_texture_ex(
                &retry_img,
                retry_pos.x,
                retry_pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(retry_size),
                    ..DrawTextureParams::default()
                },
            );
            for touch in touches {
                if touch.x > retry_pos.x && touch.y > retry_pos.y && touch.x < retry_pos.x + retry_size.x && touch.y < retry_pos.y + retry_size.y {
                    balloons = Vec::new();
                    timer = 0.0;
                    health = 10.0;
                    score = 0.0;
                    combo = 1.0;
                    game_speed = 0.5;
                    dead = false;
                    health_flash = 0.0;
                    poofs = Vec::new();
                }
            }

            next_frame().await;
            continue;
        }

        let delta = get_frame_time();
        game_speed += delta / 120.0;
        timer -= delta;
        while timer <= 0.0 {
            balloons.push((
                (rand::gen_range(100.0, screen_width() - 100.0), -90.0),
                rand::gen_range(0, balloon_textures.len()),
                rand::gen_range(200.0, 250.0),
            ));
            timer += rand::gen_range(0.0, 1.0) / game_speed.clamp(0.5, 2.0);
        }
        for (pos, variant, spd) in &mut balloons {
            pos.1 += *spd * delta * game_speed;
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

        for touch in touches {
            for i in (0..balloons.len()).rev() {
                if (Vec2::new(balloons[i].0 .0, screen_height() - balloons[i].0 .1) - touch)
                    .length_squared()
                    < 10000.0
                {
                    play_sound_once(&pop_sounds[rand::gen_range(0, pop_sounds.len())]);
                    poofs.push((Vec2::from(balloons[i].0), 0.0));
                    balloons.remove(i);
                    combo += 1.0;
                    score += (combo - 4.0).max(1.0).log2().floor().clamp(1.0, 5.0);
                    break;
                }
            }
        }
        for i in (0..poofs.len()).rev() {
            poofs[i].1 += delta*3.0;
            if poofs[i].1 > 1.0 {
                poofs.remove(i);
                continue;
            }
            let tex = &poof_textures[(poofs[i].1*3.99).floor() as usize];
            draw_texture_ex(
                tex,
                poofs[0].0.x - tex.size().x / 4.0,
                screen_height() - poofs[i].0.y - tex.size().y / 4.0,
                Color::new(1.0, 1.0, 1.0, (1.0-poofs[i].1)/3.0),
                DrawTextureParams {
                    dest_size: Some(tex.size() / 2.0),
                    ..DrawTextureParams::default()
                },
            );
        }
        for i in (0..balloons.len()).rev() {
            if balloons[i].0 .1 > screen_height() + 90.0 {
                balloons.remove(i);
                if combo == 0.0 {
                    health -= 1.0;
                    health_flash = 1.0;
                }
                combo = 0.0;
            }
        }
        if health < 0.0 {
            // Ded lol
            dead = true
        }
        health = (health + delta / 10.0).min(10.0);

        draw_texture_ex(
            &score_img,
            screen_width() - score_img.size().x / 3.0 - 20.0,
            10.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(score_img.size() / 3.0),
                ..DrawTextureParams::default()
            },
        );
        let score_text = format!("{score:.0}");
        let text_size = measure_text(&score_text, Some(&font), 60, 1.0);
        draw_text_ex(
            &score_text,
            screen_width() - 120.0 - text_size.width,
            60.0,
            TextParams {
                font: Some(&font),
                font_size: 60,
                ..TextParams::default()
            },
        );
        draw_rectangle(
            90.0,
            50.0,
            health * 35.0,
            40.0,
            Color::new(1.0, health_flash, health_flash, 1.0),
        );
        draw_rectangle(
            90.0,
            75.0,
            health * 35.0,
            15.0,
            Color::new(0.5+health_flash/2.0, health_flash, health_flash, 1.0),
        );
        health_flash = (health_flash - delta*3.0).max(0.0);
        draw_texture_ex(
            &health_img,
            20.0,
            10.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(health_img.size() / 4.0),
                ..DrawTextureParams::default()
            },
        );

        next_frame().await
    }
}
