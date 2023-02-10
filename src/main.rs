use bevy::math::vec3;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, time::FixedTimestep};
use web_sys::console;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Component)]
struct Particle;

#[derive(Debug, Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(move_particle),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
            ..default()
        },
        Velocity(Vec2::new(10.0, 10.0)),
    ));
}

fn move_particle(mut query: Query<(&mut Transform, &Velocity)>) {

    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;

        let msg = format!("{:?}", transform);
        console::log_1(&msg.into());
    }
}
