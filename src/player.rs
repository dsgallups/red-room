use std::f32::consts::{self, FRAC_PI_2};

use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use avian3d::prelude::*;
use bevy::prelude::*;

pub struct PlayGamePlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (spawn_player, make_scene))
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
    }
}

fn make_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Static physics object with a collision shape
    commands.spawn((
        RigidBody::Static,
        ColliderConstructor::TrimeshFromMesh,
        MeshMaterial3d(materials.add(Color::WHITE)),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 20.))),
    ));

    //back wall
    // commands.spawn((
    //     RigidBody::Static,
    //     ColliderConstructor::TrimeshFromMesh,
    //     MeshMaterial3d(materials.add(Color::WHITE)),
    //     Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 10.))),
    //     Transform::from_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2 * 0.8))
    //         .with_translation(Vec3::new(10., 5., 0.)),
    // ));
    //

    //back wall
    let quat_back = Quat::from_xyzw(-0.5, 0.5, 0.5, 0.5);
    commands.spawn((
        RigidBody::Static,
        ColliderConstructor::TrimeshFromMesh,
        MeshMaterial3d(materials.add(Color::WHITE)),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 10.))),
        Transform::from_rotation(quat_back).with_translation(Vec3::new(10., 5., 0.)),
    ));

    //front wall
    let quat_front = Quat::from_xyzw(0.5, 0.5, 0.5, -0.5);
    commands.spawn((
        RigidBody::Static,
        ColliderConstructor::TrimeshFromMesh,
        MeshMaterial3d(materials.add(Color::WHITE)),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 10.))),
        Transform::from_rotation(quat_front).with_translation(Vec3::new(-10., 5., 0.)),
    ));

    //left wall
    commands.spawn((
        RigidBody::Static,
        ColliderConstructor::TrimeshFromMesh,
        MeshMaterial3d(materials.add(Color::linear_rgb(0., 1., 1.))),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 10.))),
        Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2))
            .with_translation(Vec3::new(0., 5., -10.)),
    ));

    //right wall
    commands.spawn((
        RigidBody::Static,
        ColliderConstructor::TrimeshFromMesh,
        MeshMaterial3d(materials.add(Color::linear_rgb(0., 1., 0.))),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 10.))),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
            .with_translation(Vec3::new(0., 5., 10.)),
    ));

    // Dynamic physics object with a collision shape and initial angular velocity
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 4.0, 0.0),
    ));

    // Light
    for z in [-7.0, 7.0] {
        commands.spawn((
            PointLight {
                color: Color::linear_rgb(1., 0., 0.),
                range: 80.,
                shadows_enabled: true,
                ..default()
            },
            Transform::from_xyz(4., 8.0, z),
        ));
    }

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(40., 13., 7.0).looking_at(Vec3::ZERO, Dir3::Y),
    ));
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn((
        Sprite::from_image(textures.bevy.clone()),
        Transform::from_translation(Vec3::new(0., 0., 1.)),
        Player,
    ));
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_secs(),
        actions.player_movement.unwrap().y * speed * time.delta_secs(),
        0.,
    );
    for mut player_transform in &mut player_query {
        player_transform.translation += movement;
    }
}
