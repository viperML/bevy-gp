use std::iter::zip;

use bevy::math::vec3;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, time::FixedTimestep};
use bevy_asset::AssetServer;
use bevy_particle_systems::*;

#[cfg(target_arch = "wasm32")]
use web_sys::console;

use bevy_web_asset::WebAssetPlugin;

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
        .add_plugin(WebAssetPlugin::default())
        .add_plugins(DefaultPlugins.build().disable::<AssetPlugin>())
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

    // commands.spawn((
    // ));

    commands
        .spawn(StateVector(vec![-2.0, 0.0, 0.0, 1.0, 0.0]))
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
            ..default()
        }).insert(ParticleSystemBundle {
            particle_system: ParticleSystem {
                max_particles: 500,
                emitter_shape: bevy_particle_systems::EmitterShape::CircleSegment {
                    opening_angle: std::f32::consts::PI * 0.25,
                    direction_angle: std::f32::consts::PI,
                    radius: 0.0.into(),
                },
                texture: ParticleTexture::Sprite(asset_server.load("https://media.githubusercontent.com/media/abnormalbrain/bevy_particle_systems/main/assets/px.png")),
                spawn_rate_per_second: 35.0.into(),
                initial_speed: JitteredValue::jittered(25.0, 0.0..5.0),
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
            // transform: Transform::from_xyz(-100.0, 50.0, 0.0),
            transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
            ..ParticleSystemBundle::default()
        }).insert(Playing);
}

const A: [f32; 11] = [0.6, 10.0, 1.5, 1.5, 0.5, 3.8, 28.0, 0.2, 0.19, 0.9, 0.6];

fn move_particle(mut query: Query<(&mut Transform, &mut StateVector)>) {
    for (mut transform, mut x) in &mut query {
        let g: [f32; 5] = [
            A[0] * x[0] - A[1] * x[1] + A[2] * x[2] + A[3] * x[3] - A[10] * x[4],
            A[4] * x[0] - A[5] * f32::atan(A[6] * x[1]),
            -A[7] * x[0] - A[3] * x[3],
            A[8] * x[2] - A[0] * x[4],
            A[9] * x[4],
        ];

        for i in 0..4 {
            x[i] = x[i] + g[i] * TIME_STEP; // TODO dt
        }

        transform.translation.x = x[0] * 30.0;
        transform.translation.y = x[1] * 30.0;
    }
}
