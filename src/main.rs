use argh::FromArgs;
use bevy::{prelude::*, render::pass::ClearColor};
use std::collections::HashMap;

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
        let (self_x, self_y) = <(f32, f32)>::from(self.position());
        let (self_size_x, self_size_y) = <(f32, f32)>::from(self.size());
        let (other_x, other_y) = <(f32, f32)>::from(other.position());
        let (other_size_x, other_size_y) = <(f32, f32)>::from(other.size());

        if (self_x - other_x).abs() < (self_size_x / 2.0 + other_size_x / 2.0)
            && (self_y - other_y).abs() < (self_size_y / 2.0 + other_size_y / 2.0)
        {
            return true;
        }

        false
    }

    fn collision_by_axis(&self, other: &impl AABB) -> Vec2 {
        let (self_x, self_y) = <(f32, f32)>::from(self.position());
        let (self_size_x, self_size_y) = <(f32, f32)>::from(self.size());
        let (other_x, other_y) = <(f32, f32)>::from(other.position());
        let (other_size_x, other_size_y) = <(f32, f32)>::from(other.size());

        let h = (self_x - other_x).signum()
            * ((self_x - other_x).abs() - (self_size_x / 2.0 + other_size_x / 2.0));

        let v = (self_y - other_y).signum()
            * ((self_y - other_y).abs() - (self_size_y / 2.0 + other_size_y / 2.0));

        Vec2::new(h, v)
    }

    fn position(&self) -> Vec2;
    fn size(&self) -> Vec2;
}

struct Camera {}

struct Player {
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

struct Object {
    position: Vec2,
    size: Vec2,
}

impl AABB for Object {
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
    let bottom = scale * (-BG_HEIGHT / 2.0 + 8.0);

    commands
        .spawn(Camera2dComponents {
            translation: Translation::new(0.0, 0.0, 20.0),
            ..Default::default()
        })
        .with(Camera {});

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
                    acceleration: 0.2 * (5 - i) as f32,
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

    let object_texture_handle = asset_server
        .load_sync(&mut textures, "assets/tileset.png")
        .unwrap();
    let object_texture = textures.get(&object_texture_handle).unwrap();
    let object_texture_atlas =
        TextureAtlas::from_grid(object_texture_handle, object_texture.size, 48, 23);

    let run_atlas_handle = texture_atlases.add(run_texture_atlas);
    let idle_atlas_handle = texture_atlases.add(idle_texture_atlas);
    let jump_atlas_handle = texture_atlases.add(jump_texture_atlas);
    let object_atlas_handle = texture_atlases.add(object_texture_atlas);

    commands
        .spawn(SpriteSheetComponents {
            scale: Scale(scale),
            translation: Translation::new(0.0, bottom, 15.0),
            texture_atlas: idle_atlas_handle.clone(),
            ..Default::default()
        })
        .with(Player {
            size: Vec2::new(19.0 * scale, 33.0 * scale),
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
        })
        .with(Timer::from_seconds(0.1, true));

    let mut sprites = Sprites::new();

    sprites.add("player_run".to_string(), run_atlas_handle);
    sprites.add("player_idle".to_string(), idle_atlas_handle);
    sprites.add("player_jump".to_string(), jump_atlas_handle);

    commands.insert_resource(sprites);

    for i in -20..=20 {
        commands
            .spawn(SpriteSheetComponents {
                scale: Scale(scale),
                sprite: TextureAtlasSprite::new(101),
                texture_atlas: object_atlas_handle.clone(),
                translation: Translation::new(i as f32 * 16.0 * scale, bottom, 10.0),
                ..Default::default()
            })
            .with(Object {
                size: Vec2::new(16.0 * scale, 16.0 * scale),
                position: Vec2::new(i as f32 * 16.0 * scale, bottom),
            });
    }

    commands
        .spawn(SpriteSheetComponents {
            scale: Scale(scale),
            sprite: TextureAtlasSprite::new(101),
            texture_atlas: object_atlas_handle.clone(),
            translation: Translation::new(5.0 * 16.0 * scale, bottom + 16.0 * scale, 10.0),
            ..Default::default()
        })
        .with(Object {
            size: Vec2::new(16.0 * scale, 16.0 * scale),
            position: Vec2::new(5.0 * 16.0 * scale, bottom + 16.0 * scale),
        });
}

fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Player>,
    mut object_query: Query<&Object>,
) {
    for mut player in &mut player_query.iter() {
        if keyboard_input.pressed(KeyCode::Right) {
            player.velocity.set_x(PLAYER_HORIZONTAL_SPEED);
        }

        if keyboard_input.pressed(KeyCode::Left) {
            player.velocity.set_x(-PLAYER_HORIZONTAL_SPEED);
        }

        if keyboard_input.just_released(KeyCode::Right)
            || keyboard_input.just_released(KeyCode::Left)
        {
            player.velocity.set_x(0.0);
        }

        if keyboard_input.pressed(KeyCode::Up) {
            if player.velocity.y() == 0.0 {
                player.velocity.set_y(PLAYER_INITIAL_VERTICAL_SPEED);
            }
        }

        // player is constantly affected by gravity
        *player.velocity.y_mut() -= GRAVITY * time.delta_seconds;

        *player.position.x_mut() += player.velocity.x();
        *player.position.y_mut() += player.velocity.y();

        for object in &mut object_query.iter() {
            if player.collides(object) {
                let collision = player.collision_by_axis(object);

                if collision.y().abs() < collision.x().abs() {
                    let sign_y = player.velocity.y().signum();

                    *player.position.y_mut() = object.position.y()
                        - sign_y * (object.size.y() / 2.0 + player.size.y() / 2.0);

                    player.velocity.set_y(0.0);
                } else {
                    let sign_x = collision.x().signum();

                    *player.position.x_mut() = object.position.x()
                        - sign_x * (object.size.x() / 2.0 + player.size.x() / 2.0);

                    player.velocity.set_x(0.0);
                }
            }
        }
    }
}

fn animation(
    options: Res<Options>,
    sprites: Res<Sprites>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut player_query: Query<(
        &Player,
        &mut Rotation,
        &mut Translation,
        &mut Timer,
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
    )>,
    mut camera_query: Query<(&Camera, &mut Translation)>,
    mut background_query: Query<(&Background, &mut Translation)>,
) {
    let scale = options.scale as f32;

    for (player, mut rotation, mut translation, timer, mut sprite, mut texture_atlas_handle) in
        &mut player_query.iter()
    {
        *translation.x_mut() = player.position.x();
        *translation.y_mut() = player.position.y();

        if player.velocity.x() != 0.0 {
            if let Some(sprite_handle) = sprites.get("player_run") {
                *texture_atlas_handle = *sprite_handle;
            }

            if player.velocity.x() > 0.0 {
                *rotation = Rotation(Quat::from_rotation_y(0.0));
            } else {
                *rotation = Rotation(Quat::from_rotation_y(std::f32::consts::PI));
            }
        } else {
            if let Some(sprite_handle) = sprites.get("player_idle") {
                *texture_atlas_handle = *sprite_handle;
            }
        }

        if player.velocity.y() != 0.0 {
            if let Some(sprite_handle) = sprites.get("player_jump") {
                *texture_atlas_handle = *sprite_handle;
            }
        }

        if timer.finished {
            let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }

        for (_, mut translation) in &mut camera_query.iter() {
            translation.set_x(player.position.x());
        }

        for (background, mut translation) in &mut background_query.iter() {
            *translation.0.x_mut() += player.velocity.x() * background.acceleration;

            if player.position.x() - translation.0.x() > BG_WIDTH * scale {
                *translation.0.x_mut() += 2.0 * BG_WIDTH * scale;
            } else if player.position.x() - translation.0.x() < -BG_WIDTH * scale {
                *translation.0.x_mut() -= 2.0 * BG_WIDTH * scale;
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
