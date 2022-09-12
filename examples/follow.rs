use std::f32::consts::PI;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::prelude::shape::Quad;
use bevy_dolly::{drivers::follow::MovableLookAt, prelude::*};
use bevy_dolly::prelude::cursor_grab::DollyCursorGrab;

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })

        .insert_resource(NewPosition(Vec3::ZERO))
        .add_plugins(DefaultPlugins)
        .add_plugin(DollyCursorGrab)
        .add_dolly_component(MainCamera)
        .add_startup_system(setup)
        .add_system(rotator_system)
        .add_system(update_camera)
        .add_system(bevy::window::close_on_esc)
        .run();
}

struct NewPosition(Vec3);

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    let start_pos = dolly::glam::Vec3::new(0., 0., 0.);


    commands
        .spawn_bundle(SceneBundle {
            scene: asset_server.load("poly_dolly.gltf#Scene0"),
            transform: Transform {
                translation: Vec3::new(0., 0.2, 0.),
                ..default()
            },
            ..default()
        })
        .insert(Rotates);

    commands
        .spawn()
        .insert(
            Rig::builder()
                .with(Position::new(start_pos))
                .with(Rotation::new(dolly::glam::Quat::IDENTITY))
                .with(Smooth::new_position(1.25).predictive(true))
                .with(Arm::new(dolly::glam::Vec3::new(0.0, 1.5, -3.5)))
                .with(Smooth::new_position(0.5))
                .with(YawPitch::new().yaw_degrees(180.0).pitch_degrees(-10.0))
                .with(Smooth::new_rotation(1.5))
                .with(
                    LookAt::new(start_pos + dolly::glam::Vec3::Y)
                        .tracking_smoothness(1.25)
                        .tracking_predictive(true),
                )
                .build(),
                // .with(MovableLookAt::from_position_target(start_pos))
                // // .with(Position::new(Vec3::new(-2.0, 1., 5.0)))
                // // .with(Arm::new(dolly::glam::Vec3::new(0.0, 1.5, -3.5)))
                // .with(Smooth::new_rotation(1.5))
                // .build(),
        )
        .insert(MainCamera);

    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 1., 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(MainCamera);

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn update_camera(
    q0: Query<(&Transform, With<Rotates>)>, 
    mut q1: Query<&mut Rig>,
    position: Res<NewPosition>
) {
    let player = q0.single().0.to_owned();
    let mut rig = q1.single_mut();

    let target_position = player.translation;
    let target_rotation = player.rotation;

    rig.driver_mut::<Position>().position = target_position;
    rig.driver_mut::<Rotation>().rotation = target_rotation;
    rig.driver_mut::<LookAt>().target = target_position + dolly::glam::Vec3::Y;

    rig.driver_mut::<Arm>().offset.y += position.0.y;


    rig.driver_mut::<Arm>().offset.y = rig.driver_mut::<Arm>().offset.y.clamp(
        1.0,
        5.0,
    );
}

#[derive(Component)]
struct Rotates;

fn rotator_system(
    time: Res<Time>, 
    mut object_query: Query<&mut Transform, With<Rotates>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut windows: ResMut<Windows>,
    mut camera_query: Query<&mut Rig>,
    mut camera2_query: Query<&mut Transform, (With<MainCamera>, Without<Rotates>)>,
    mut position: ResMut<NewPosition>
) {
    let mut window = windows.get_primary_mut().unwrap();
    let mut object_transform = object_query.single_mut();
    window.set_cursor_lock_mode(true);
    window.set_cursor_visibility(false);

    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        delta += event.delta;
    }


    let mut rig = camera_query.single_mut();
    let camera_driver = rig.driver_mut::<YawPitch>();

    let mut camera_transform = camera2_query.single_mut();

    let sensitivity = Vec2::new(
        0.0005,
        0.001
    );

    let yaw = delta.x * sensitivity.x; 
    let pitch = delta.y * sensitivity.y;

    camera_driver.rotate_yaw_pitch(
        yaw.to_degrees(),
        0.0
    );

    position.0.y = pitch;

    *object_transform = Transform::from_rotation(Quat::from_rotation_y(
        yaw,
    )) * *object_transform;

    // camera_transform.translation.y += pitch;

    println!("Object: {}", object_transform.rotation.y);
    println!("Camera: {}", camera_transform.translation.y);

    // for mut transform in query.iter_mut() {
        // *transform = Transform::from_rotation(Quat::from_rotation_y(
        //     (4.0 * PI / 20.0) * time.delta_seconds(),
        // )) * *transform;
    // }
}
