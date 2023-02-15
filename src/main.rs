use std::iter::zip;

use bevy::math::vec3;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, time::FixedTimestep};
use bevy_asset::AssetServer;
use bevy_particle_systems::*;

#[cfg(target_arch = "wasm32")]
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
    asset_server: Res<AssetServer>,
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

    commands.spawn((
        StateVector(vec![-2.0, 0.0, 0.0, 1.0, 0.0]),
        ParticleSystemBundle {
            particle_system: ParticleSystem {
                max_particles: 500,
                emitter_shape: bevy_particle_systems::EmitterShape::CircleSegment {
                    opening_angle: std::f32::consts::PI * 0.25,
                    direction_angle: std::f32::consts::PI,
                    radius: 0.0.into(),
                },
                texture: ParticleTexture::Sprite(asset_server.load("px.png")),
                spawn_rate_per_second: 35.0.into(),
                initial_speed: JitteredValue::jittered(25.0, 0.0..0.0),
                acceleration: 0.0.into(),
                lifetime: JitteredValue::jittered(3.0, -2.0..2.0),
                color: ColorOverTime::Gradient(Gradient::new(vec![
                    ColorPoint::new(Color::GREEN, 0.0),
                    ColorPoint::new(Color::rgba(0.0, 0.0, 0.0, 0.0), 1.0),
                ])),
                looping: true,
                system_duration_seconds: 10.0,
                space: ParticleSpace::Local,
                scale: 8.0.into(),
                rotation_speed: JitteredValue::jittered(0.0, -6.0..0.0),
                ..ParticleSystem::default()
            },
            ..ParticleSystemBundle::default()
        },
    ));
}

fn move_particle(mut query: Query<(&mut Transform, &mut StateVector)>) {
    for (mut transform, mut x) in &mut query {
        // Transfer parameters
        let a: [f32; 11] = [0.6, 10.0, 1.5, 1.5, 0.5, 3.8, 28.0, 0.2, 0.19, 0.9, 0.6];

        let g: [f32; 5] = [
            a[0] * x[0] - a[1] * x[1] + a[2] * x[2] + a[3] * x[3] - a[10] * x[4],
            a[4] * x[0] - a[5] * f32::atan(a[6] * x[1]),
            -a[7] * x[0] - a[3] * x[3],
            a[8] * x[2] - a[0] * x[4],
            a[9] * x[4],
        ];

        let msg = format!("{:?}", g);
        // console::log_1(&msg.into());

        for i in 0..4 {
            x[i] = x[i] + g[i] * TIME_STEP; // TODO dt
        }

        transform.translation.x = x[0] * 30.0;
        transform.translation.y = x[1] * 30.0;
    }
    // for (mut transform, velocity) in &mut query {
    //     transform.translation.x += velocity.x * TIME_STEP;
    //     transform.translation.y += velocity.y * TIME_STEP;

    //     let msg = format!("{:?}", transform);
    //     console::log_1(&msg.into());
    // }
}
