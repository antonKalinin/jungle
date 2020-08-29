use argh::FromArgs;
use bevy::{prelude::*, render::pass::ClearColor};

#[derive(FromArgs)]
#[argh(description = "Jungle game settings")]
struct Options {
    #[argh(
        option,
        default = "4",
        short = 's',
        description = "scale of game window"
    )]
    scale: u32,
}

#[derive(Default)]
struct GameState {}

struct Background {
    acceleration: f32,
}

struct Player {
    velocity: f32,
}

const BG_WIDTH: f32 = 384.0;
const BG_HEIGHT: f32 = 216.0;
const PLAYER_SPEED: f32 = 10.0;

/**
 * Setup system responsible for:
 *  - loading sprites: backgrounds, player
 *  - setting sprite components initial position
 **/
fn startup(
    mut commands: Commands,
    options: Res<Options>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let scale = options.scale as f32;

    // Add the game's entities to our world
    commands.spawn(Camera2dComponents::default());

    for i in 1..=5 {
        let bg_handle = asset_server
            .load(format!("assets/background/plx-{}.png", i))
            .unwrap();

        for j in 0..=1 {
            commands
                .spawn(SpriteComponents {
                    scale: Scale(scale),
                    material: materials.add(bg_handle.into()),
                    translation: Translation::new(scale * BG_WIDTH * j as f32, 0.0, i as f32),
                    ..Default::default()
                })
                .with(Background {
                    acceleration: 0.2 * i as f32,
                });
        }
    }

    commands
        .spawn(SpriteComponents {
            scale: Scale(scale),
            translation: Translation::new(0.0, 0.0, 10.0),
            ..Default::default()
        })
        .with(Player { velocity: 0.0 });
}

fn movement(
    options: Res<Options>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Player>,
    mut bg_query: Query<(&Background, &mut Translation)>,
) {
    let scale = options.scale as f32;

    for mut player in &mut player_query.iter() {
        if keyboard_input.pressed(KeyCode::Left) {
            player.velocity = PLAYER_SPEED;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            player.velocity = -PLAYER_SPEED;
        }

        if keyboard_input.just_released(KeyCode::Right)
            || keyboard_input.just_released(KeyCode::Left)
        {
            player.velocity = 0.0;
        }

        for (background, mut translation) in &mut bg_query.iter() {
            let step = player.velocity * background.acceleration;

            *translation.0.x_mut() += step;

            if translation.0.x() > BG_WIDTH * scale {
                *translation.0.x_mut() -= 2.0 * BG_WIDTH * scale;
            } else if translation.0.x() < -BG_WIDTH * scale {
                *translation.0.x_mut() += 2.0 * BG_WIDTH * scale;
            }
        }
    }
}

fn main() {
    let options: Options = argh::from_env();
    let window_width = options.scale * BG_WIDTH as u32;
    let window_height = options.scale * BG_HEIGHT as u32;

    let window = WindowDescriptor {
        title: "Jungle".to_string(),
        width: window_width,
        height: window_height,
        ..Default::default()
    };

    App::build()
        .add_resource(window)
        .add_resource(options)
        .add_resource(ClearColor(Color::rgb(0.01, 0.01, 0.01)))
        .add_default_plugins()
        .init_resource::<GameState>()
        // Startup systems run exactly once BEFORE all other systems. These are generally used for
        // app initialization code (ex: adding entities and resources)
        .add_startup_system(startup.system())
        .add_system(movement.system())
        .run();
}
