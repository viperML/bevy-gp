use bevy::math::vec3;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, time::FixedTimestep};
use bevy_particle_systems::*;
use web_sys::console;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Component)]
struct Particle;

#[derive(Debug, Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Debug, Component, Deref, DerefMut)]
struct StateVector(Vec<f32>);

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(ParticleSystemPlugin::default())
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
        StateVector(vec![-2.0, 0.0, 0.0, 1.0, 0.0]),
    ));

    // commands.spawn(ParticleSystemBundle {
    //     particle_system: ParticleSystem {
    //         max_particles: 10,
    //         spawn_rate_per_second: 25.0.into(),
    //         initial_speed: JitteredValue {
    //             value: 0.0,
    //             jitter_range: None,
    //         },
    //         lifetime: JitteredValue {
    //             value: 8.0,
    //             jitter_range: None,
    //         },
    //     },
    //     ..ParticleSystemBundle::default()
    // });
}

fn move_particle(mut query: Query<(&mut Transform, &mut StateVector)>) {
    for (mut transform, mut state_vector) in &mut query {
        let transfer_params: [f64; 10] = [0.6, 10.0, 1.5, 0.5, 3.8, 28.0, 0.2, 0.19, 0.9, 0.6];

        // let transfer = [
        //     transfer_params[0]*state_vector[]
        // ];

    }
    // for (mut transform, velocity) in &mut query {
    //     transform.translation.x += velocity.x * TIME_STEP;
    //     transform.translation.y += velocity.y * TIME_STEP;

    //     let msg = format!("{:?}", transform);
    //     console::log_1(&msg.into());
    // }
}
