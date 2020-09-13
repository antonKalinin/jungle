use argh::FromArgs;
use bevy::{prelude::*, render::pass::ClearColor};
use std::collections::HashMap;

const AXIS_X: usize = 0;
const AXIS_Y: usize = 1;
const BG_WIDTH: f32 = 384.0;
const BG_HEIGHT: f32 = 216.0;
const GRAVITY: f32 = 32.0;
const PLAYER_HORIZONTAL_SPEED: f32 = 8.0;
const PLAYER_INITIAL_VERTICAL_SPEED: f32 = 16.0;

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

trait AABB {
    fn collides(&self, other: &impl AABB) -> bool {
        let self_position = self.position();
        let self_size = self.size();
        let other_position = other.position();
        let other_size = other.size();

        if self_position[AXIS_X] - other_position[AXIS_X]
            > self_size[AXIS_X] / 2.0 + other_size[AXIS_X] / 2.0
        {
            return false;
        }

        if self_position[AXIS_Y] - other_position[AXIS_Y]
            > self_size[AXIS_Y] / 2.0 + other_size[AXIS_Y] / 2.0
        {
            return false;
        }

        true
    }
    fn position(&self) -> Vec2;
    fn size(&self) -> Vec2;
}

struct Player {
    scale: f32,
    position: Vec2,
    size: Vec2,
    velocity: Vec2,
}

impl AABB for Player {
    fn position(&self) -> Vec2 {
        self.position.clone()
    }

    fn size(&self) -> Vec2 {
        self.size.clone()
    }
}

struct Tile {
    scale: f32,
    position: Vec2,
    size: Vec2,
}

impl AABB for Tile {
    fn position(&self) -> Vec2 {
        self.position.clone()
    }

    fn size(&self) -> Vec2 {
        self.size.clone()
    }
}

pub struct Sprites {
    library: HashMap<String, Handle<TextureAtlas>>,
}

impl Sprites {
    pub fn new() -> Sprites {
        Sprites {
            library: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: String, atlas_handle: Handle<TextureAtlas>) {
        self.library.insert(key, atlas_handle);
    }

    pub fn get(&self, key: &str) -> Option<&Handle<TextureAtlas>> {
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
    let bottom = scale * (-BG_HEIGHT / 2.0 + 20.0);

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

    let jump_texture_handle = asset_server
        .load_sync(&mut textures, "assets/player/jump.png")
        .unwrap();
    let jump_texture = textures.get(&jump_texture_handle).unwrap();
    let jump_texture_atlas = TextureAtlas::from_grid(jump_texture_handle, jump_texture.size, 1, 1);

    let tile_texture_handle = asset_server
        .load_sync(&mut textures, "assets/tileset.png")
        .unwrap();
    let tile_texture = textures.get(&tile_texture_handle).unwrap();
    let tile_texture_atlas =
        TextureAtlas::from_grid(tile_texture_handle, tile_texture.size, 48, 23);

    let run_atlas_handle = texture_atlases.add(run_texture_atlas);
    let idle_atlas_handle = texture_atlases.add(idle_texture_atlas);
    let jump_atlas_handle = texture_atlases.add(jump_texture_atlas);
    let tile_atlas_handle = texture_atlases.add(tile_texture_atlas);

    commands
        .spawn(SpriteSheetComponents {
            scale: Scale(scale),
            translation: Translation::new(0.0, bottom, 10.0),
            texture_atlas: idle_atlas_handle.clone(),
            ..Default::default()
        })
        .with(Player {
            scale,
            size: Vec2::new(34.0, 20.0),
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
        })
        .with(Timer::from_seconds(0.1, true));

    let mut sprites = Sprites::new();

    sprites.add("player_run".to_string(), run_atlas_handle);
    sprites.add("player_idle".to_string(), idle_atlas_handle);
    sprites.add("player_jump".to_string(), jump_atlas_handle);

    commands.insert_resource(sprites);

    commands
        .spawn(SpriteSheetComponents {
            scale: Scale(scale),
            sprite: TextureAtlasSprite::new(101),
            texture_atlas: tile_atlas_handle.clone(),
            translation: Translation::new(100.0, bottom - 10.0, 11.0),
            ..Default::default()
        })
        .with(Tile {
            scale,
            size: Vec2::new(16.0, 16.0),
            position: Vec2::new(10.0, 0.0),
        });
}

fn movement(
    time: Res<Time>,
    options: Res<Options>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Translation, &mut Rotation)>,
) {
    let bottom = options.scale as f32 * (-BG_HEIGHT / 2.0 + 20.0);

    for (mut player, mut translation, mut rotation) in &mut player_query.iter() {
        if keyboard_input.pressed(KeyCode::Left) {
            player.velocity[AXIS_X] = PLAYER_HORIZONTAL_SPEED;
            *rotation = Rotation(Quat::from_rotation_y(std::f32::consts::PI));
        }

        if keyboard_input.pressed(KeyCode::Right) {
            player.velocity[AXIS_X] = -PLAYER_HORIZONTAL_SPEED;
            *rotation = Rotation(Quat::from_rotation_y(0.0));
        }

        if keyboard_input.just_released(KeyCode::Up) {
            player.velocity[AXIS_Y] = PLAYER_INITIAL_VERTICAL_SPEED;
        }

        if keyboard_input.just_released(KeyCode::Right)
            || keyboard_input.just_released(KeyCode::Left)
        {
            player.velocity[AXIS_X] = 0.0;
        }

        if player.velocity[AXIS_Y] != 0.0 {
            translation[AXIS_Y] = translation[AXIS_Y] + player.velocity[AXIS_Y];
        }

        if translation[AXIS_Y] > bottom {
            player.velocity[AXIS_Y] -= GRAVITY * time.delta_seconds;
        } else {
            player.velocity[AXIS_Y] = 0.0;
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
        if player.velocity[AXIS_X] != 0.0 {
            if let Some(sprite_handle) = sprites.get("player_run") {
                *texture_atlas_handle = *sprite_handle;
            }
        } else {
            if let Some(sprite_handle) = sprites.get("player_idle") {
                *texture_atlas_handle = *sprite_handle;
            }
        }

        if player.velocity[AXIS_Y] != 0.0 {
            if let Some(sprite_handle) = sprites.get("player_jump") {
                *texture_atlas_handle = *sprite_handle;
            }
        }

        if timer.finished {
            let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }

        for (background, mut translation) in &mut bg_query.iter() {
            let step = player.velocity[AXIS_X] * background.acceleration;

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
