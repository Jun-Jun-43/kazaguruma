use bevy::{
    core_pipeline::{
        bloom::{BloomCompositeMode, BloomPrefilterSettings, BloomSettings},
        tonemapping::Tonemapping,
    },
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_resource::PrimitiveTopology,
        view::screenshot::ScreenshotManager,
    },
    window::{PrimaryWindow, WindowResolution},
};

use std::f32::consts::TAU;

fn main() {
    let window = WindowResolution::new(720.0, 1280.0).with_scale_factor_override(1.0);

    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: window,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup, directional_light_setup, camera_setup))
        .add_systems(Update, (rotate, animate_light_direction))
        // .add_systems(PostUpdate, screenshots)
        .run();
}

const SHINY_METALLIC_BLUE: Color = Color::rgb_linear(55.0 / 256.0, 48.0 / 256.0, 242.0 / 256.0);
const CIRCUIT_BOARD_GREEN: Color = Color::rgb_linear(65.0 / 256.0, 191.0 / 256.0, 73.0 / 256.0);
const LED_WHITE: Color = Color::rgb_linear(242.0 / 242.0, 242.0 / 242.0, 242.0);
const CAMERA_ANIMATION_DURATION: f32 = 0.8;

#[derive(Component)]
struct Wing;

impl Wing {
    fn new(
        commands: & mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        att_position: VertexAttributeValues,
        att_normal: VertexAttributeValues,
        color: Color,
        transform_z: f32,
    ) {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(
                    Mesh::new(PrimitiveTopology::TriangleList)
                        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, att_position)
                        // .with_inserted_attribute(
                        //     Mesh::ATTRIBUTE_UV_0,
                        //     vec![[0.0, 1.0], [0.5, 0.0], [1.0, 0.0], [0.5, 1.0]],
                        // )
                        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, att_normal)
                        .with_indices(Some(Indices::U32(vec![0, 3, 1, 1, 3, 2]))),
                ),
                material: materials.add(StandardMaterial {
                    base_color: color,
                    perceptual_roughness: 0.1,
                    metallic: 1.0,
                    ..default()
                }),
                // transform: Transform::from_xyz(0.0, 0.0, 0.0),
                transform: Transform::from_xyz(0.0, 0.0, transform_z),
                ..default()
            },
            Wing,
        ));
    }
}

#[derive(Component)]
struct Kazaguruma;

impl Kazaguruma {
    fn new(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        color: Color,
        transform_z: f32,
    ) {
        let att_position = VertexAttributeValues::Float32x3(vec![
            [0.0, 0.0, 0.0],
            [0.5, 2.0, 0.0],
            [1.0, 2.0, 0.0],
            [0.0, 0.0, 0.0],
        ]);

        let att_normal = VertexAttributeValues::Float32x3(vec![
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ]);

        for _ in 0..5 {
            Wing::new(
                commands,
                meshes,
                materials,
                att_position.clone(),
                att_normal.clone(),
                color,
                transform_z,
            );
        }

        point_light_setup(commands, color, 0.0, 0.0);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let num_kazaguruma = 2;
    let transform_z = -5.0f32;

    // Kazaguruma::new(
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    //     win.clone(),
    //     SHINY_METALLIC_BLUE,
    //     transform_z,
    // );

    Kazaguruma::new(
        &mut commands,
        &mut meshes,
        &mut materials,
        SHINY_METALLIC_BLUE,
        transform_z,
    );
}

fn directional_light_setup(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        directional_light: DirectionalLight {
            color: LED_WHITE,
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}

fn point_light_setup(commands: &mut Commands, color: Color, transform_x: f32, transform_y: f32) {
    info!(transform_x);
    commands.spawn(SpotLightBundle {
        transform: Transform::from_xyz(transform_x, transform_y, 0.0),
        spot_light: SpotLight {
            color: color,
            intensity: 80000.0,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}

fn camera_setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            transform: Transform::from_xyz(0.0, 0.0, 10.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        },
        BloomSettings {
            intensity: 0.2,
            low_frequency_boost: 0.2,
            low_frequency_boost_curvature: 1.0,
            high_pass_frequency: 0.5,
            prefilter_settings: BloomPrefilterSettings {
                threshold: 0.4,
                threshold_softness: 0.5,
            },
            composite_mode: BloomCompositeMode::Additive,
        },
    ));
}

fn rotate(mut wings: Query<&mut Transform, With<Wing>>, time: Res<Time>) {
    let count = time.elapsed_seconds() as usize;
    for (index, mut transform) in wings.iter_mut().enumerate() {
        transform.rotate_z(0.2 * TAU * time.delta_seconds());
        if index == count {
            break;
        }
    }
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotate_x(time.delta_seconds() * CAMERA_ANIMATION_DURATION);
    }
}

fn screenshots(
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    mut counter: Local<u32>,
) {
    let path = format!("./screenshots/{}.png", *counter);
    *counter += 1;

    screenshot_manager
        .save_screenshot_to_disk(main_window.single(), path)
        .unwrap();
}
