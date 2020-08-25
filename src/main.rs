use argh::FromArgs;
use bevy::{prelude::*, render::pass::ClearColor};

#[derive(FromArgs)]
#[argh(description = "Jungle game settings")]
struct Options {
    #[argh(
        option,
        default = "1792",
        short = 'w',
        description = "initial width of spawned window"
    )]
    width: u32,
    #[argh(
        option,
        default = "1120",
        short = 'h',
        description = "initial height of spawned window"
    )]
    height: u32,
}

#[derive(Default)]
struct GameState {}

/**
 * Setup system responsible for:
 *  - loading sprites: background, player
 **/
fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Add the game's entities to our world

    let bg_handle = asset_server.load("assets/background/plx-1.png").unwrap();

    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(bg_handle.into()),
            ..Default::default()
        });
}

fn main() {
    let options: Options = argh::from_env();
    let window = WindowDescriptor {
        title: "Jungle".to_string(),
        width: options.width,
        height: options.height,
        ..Default::default()
    };

    App::build()
        // .add_resource(window)
        .add_default_plugins()
        .add_resource(options)
        .init_resource::<GameState>()
        // Startup systems run exactly once BEFORE all other systems. These are generally used for
        // app initialization code (ex: adding entities and resources)
        .add_startup_system(startup.system())
        .run();
}
