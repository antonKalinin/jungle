use argh::FromArgs;
use bevy::{prelude::*, render::pass::ClearColor};
use std::collections::HashMap;

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
const PLAYER_SPEED: f32 = 8.0;

#[derive(Clone, Properties, Debug, Default)]
pub struct Sprite {
    pub atlas_handle: Handle<TextureAtlas>,
}

pub struct Sprites {
    library: HashMap<String, Sprite>,
}

impl Sprites {
    pub fn new() -> Sprites {
        Sprites {
            library: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: String, atlas_handle: Handle<TextureAtlas>) {
        let sprite = Sprite { atlas_handle };

        self.library.insert(key, sprite);
    }

    pub fn get(&self, key: &str) -> Option<&Sprite> {
        self.library.get(key)
    }
}

/**
 * Setup system responsible for:
 *  - loading sprites: backgrounds, player
 *  - setting sprite components initial position
 **/
fn startup(
    mut commands: Commands,
    options: Res<Options>,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
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

    let run_texture_handle = asset_server
        .load_sync(&mut textures, "assets/player/run.png")
        .unwrap();
    let run_texture = textures.get(&run_texture_handle).unwrap();
    let run_texture_atlas = TextureAtlas::from_grid(run_texture_handle, run_texture.size, 8, 1);

    let idle_texture_handle = asset_server
        .load_sync(&mut textures, "assets/player/idle.png")
        .unwrap();
    let idle_texture = textures.get(&idle_texture_handle).unwrap();
    let idle_texture_atlas = TextureAtlas::from_grid(idle_texture_handle, idle_texture.size, 12, 1);

    let run_atlas_handle = texture_atlases.add(run_texture_atlas);
    let idle_atlas_handle = texture_atlases.add(idle_texture_atlas);

    commands
        .spawn(SpriteSheetComponents {
            scale: Scale(scale),
            translation: Translation::new(0.0, -BG_HEIGHT - 100.0, 10.0),
            texture_atlas: idle_atlas_handle.clone(),
            ..Default::default()
        })
        .with(Player { velocity: 0.0 })
        .with(Timer::from_seconds(0.1, true));

    let mut sprites = Sprites::new();

    sprites.add("player_run".to_string(), run_atlas_handle);
    sprites.add("player_idle".to_string(), idle_atlas_handle);

    commands.insert_resource(sprites);
}

fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Rotation)>,
) {
    for (mut player, mut rotation) in &mut player_query.iter() {
        if keyboard_input.pressed(KeyCode::Left) {
            player.velocity = PLAYER_SPEED;
            *rotation = Rotation(Quat::from_rotation_y(std::f32::consts::PI));
        }

        if keyboard_input.pressed(KeyCode::Right) {
            player.velocity = -PLAYER_SPEED;
            *rotation = Rotation(Quat::from_rotation_y(0.0));
        }

        if keyboard_input.just_released(KeyCode::Right)
            || keyboard_input.just_released(KeyCode::Left)
        {
            player.velocity = 0.0;
        }
    }
}

fn animation(
    options: Res<Options>,
    sprites: Res<Sprites>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut player_query: Query<(
        &mut Player,
        &mut Timer,
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
    )>,
    mut bg_query: Query<(&Background, &mut Translation)>,
) {
    let scale = options.scale as f32;

    for (player, timer, mut sprite, mut texture_atlas_handle) in &mut player_query.iter() {
        if player.velocity != 0.0 {
            if let Some(sprite) = sprites.get("player_run") {
                *texture_atlas_handle = sprite.atlas_handle;
            }
        } else {
            if let Some(sprite) = sprites.get("player_idle") {
                *texture_atlas_handle = sprite.atlas_handle;
            }
        }

        if timer.finished {
            let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
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
        .add_system(animation.system())
        .run();
}
