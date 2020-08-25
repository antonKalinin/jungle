use argh::FromArgs;
use bevy::{
    input::mouse::MouseButtonInput,
    prelude::*,
    render::camera::{Camera, OrthographicProjection},
    render::pass::ClearColor,
    window::CursorMoved,
};
use rand::Rng;

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

struct Body {
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
}

#[derive(Default)]
struct State {
    bodies: Vec<Body>,
    cursor_position: Option<Vec2>,
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

// Util for generating bodies
fn gen_random_body(max_x: f32, max_y: f32, z: f32) -> Body {
    let mut rng = rand::thread_rng();

    Body {
        x: rng.gen_range(max_x / -2.0, max_x / 2.0),
        y: rng.gen_range(max_y / -2.0, max_y / 2.0),
        z,
        radius: rng.gen_range(0.05, 0.5),
    }
}

// System for adding bodies to scene
fn add_bodies(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    options: Res<Options>,
    mut state: ResMut<State>,
) {
    let mut rng = rand::thread_rng();
    let texture = asset_server.load("assets/circle.png").unwrap();

    commands.spawn(Camera2dComponents {
        scale: Scale(1.0),
        ..Camera2dComponents::default()
    });

    state.bodies = vec![];

    for n in 0..10 {
        state.bodies.push(gen_random_body(
            options.width as f32,
            options.height as f32,
            n as f32,
        ));
    }

    for body in state.bodies.iter() {
        commands.spawn(SpriteComponents {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
                body.radius,
                body.radius,
            )))),
            material: materials.add(ColorMaterial {
                color: Color::rgb(
                    rng.gen_range(0.8, 1.),
                    rng.gen_range(0., 0.6),
                    rng.gen_range(0., 0.05),
                ),
                texture: texture.into(),
            }),
            translation: Translation(Vec3::new(body.x, body.y, body.z)),
            draw: Draw {
                is_transparent: true,
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

fn get_body_position(body: &Body, screen_width: f32, screen_height: f32) -> Vec2 {
    let x = screen_width / 2.0 + body.x;
    let y = screen_height / 2.0 + body.y;

    Vec2::new(x, y)
}

fn is_cursor_in_body(cursor_position: Option<Vec2>, body_position: Vec2, body_radius: f32) -> bool {
    match cursor_position {
        None => false,
        Some(position) => {
            (body_radius * 100.0).powf(2.0)
                > ((position[0] - body_position[0]).powf(2.0)
                    + (position[1] - body_position[1]).powf(2.0))
        }
    }
}

// System to control mouse events
fn mouse_input(
    mut state: ResMut<State>,
    options: Res<Options>,
    mouse_button_input_events: Res<Events<MouseButtonInput>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
) {
    for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
        state.cursor_position = Some(event.position);
    }

    for event in state
        .mouse_button_event_reader
        .iter(&mouse_button_input_events)
    {
        if event.state.is_pressed() {
            for body in state.bodies.iter() {
                let body_position =
                    get_body_position(body, options.width as f32, options.height as f32);

                if is_cursor_in_body(state.cursor_position, body_position, body.radius) {
                    println!("In Body!");
                }
            }
        }
    }
}

fn main() {
    let options: Options = argh::from_env();
    let window = WindowDescriptor {
        title: "Shooty".to_string(),
        width: options.width,
        height: options.height,
        ..Default::default()
    };

    App::build()
        .add_resource(window)
        .add_resource(ClearColor(Color::rgb(0.01, 0.01, 0.01)))
        .add_default_plugins()
        .add_resource(options)
        .init_resource::<State>()
        .add_startup_system(add_bodies.system())
        .add_system(mouse_input.system())
        .run();
}
