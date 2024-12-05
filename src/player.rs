use crate::dynamic_character_plugin::CharacterControllerBundle;
use crate::GameState;
use avian3d::math::Scalar;
use avian3d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub struct PlayGamePlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), make_scene)
            .add_systems(Update, focus_camera.run_if(in_state(GameState::Playing)));
    }
}

fn make_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //floor
    commands.spawn((
        RigidBody::Static,
        ColliderConstructor::TrimeshFromMesh,
        MeshMaterial3d(materials.add(Color::WHITE)),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 20.))),
    ));

    //right wall
    let quat_right = Quat::from_xyzw(-0.5, 0.5, 0.5, 0.5);
    commands.spawn((
        RigidBody::Static,
        ColliderConstructor::TrimeshFromMesh,
        MeshMaterial3d(materials.add(Color::WHITE)),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 10.))),
        Transform::from_rotation(quat_right).with_translation(Vec3::new(10., 5., 0.)),
    ));

    //left wall
    let quad_left = Quat::from_xyzw(0.5, 0.5, 0.5, -0.5);
    commands.spawn((
        RigidBody::Static,
        ColliderConstructor::TrimeshFromMesh,
        MeshMaterial3d(materials.add(Color::linear_rgb(1., 1., 0.))),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 10.))),
        Transform::from_rotation(quad_left).with_translation(Vec3::new(-10., 5., 0.)),
    ));

    //back wall
    commands.spawn((
        RigidBody::Static,
        ColliderConstructor::TrimeshFromMesh,
        MeshMaterial3d(materials.add(Color::linear_rgb(0., 1., 1.))),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 10.))),
        Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2))
            .with_translation(Vec3::new(0., 5., -10.)),
    ));

    //front wall
    commands.spawn((
        RigidBody::Static,
        ColliderConstructor::TrimeshFromMesh,
        MeshMaterial3d(materials.add(Color::linear_rgb(0., 1., 0.))),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 10.))),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
            .with_translation(Vec3::new(0., 5., 10.)),
    ));

    // Player
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(0.4, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 1.5, 0.0),
        CharacterControllerBundle::new(Collider::capsule(0.4, 1.0)).with_movement(
            30.0,
            0.92,
            7.0,
            (30.0 as Scalar).to_radians(),
        ),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        GravityScale(2.0),
        Player,
    ));

    // Cube
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 4.0, 0.0),
    ));

    // Sphere
    commands.spawn((
        RigidBody::Dynamic,
        Collider::sphere(0.5),
        AngularVelocity(Vec3::new(2.2, 3.5, 1.5)),
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(3.0, 4.0, 0.0),
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
        Transform::from_xyz(0., 15., 25.0).looking_at(Vec3::ZERO, Dir3::Y),
    ));
}

fn focus_camera(
    time: Res<Time>,
    player: Query<&Transform, (With<Player>, Without<Camera3d>)>,
    mut camera: Query<&mut Transform, With<Camera3d>>,
) {
    const SPEED: f32 = 2.0;
    let player_t = player.get_single().unwrap().translation;

    let mut camera_transform = camera.get_single_mut().unwrap();
    let mut camera_looking_at = camera_transform.translation;
    let mut camera_motion = player_t - camera_looking_at;
    if camera_motion.length() > 0.4 {
        camera_motion *= SPEED * time.delta_secs();
        camera_looking_at += camera_motion;
    }
    // look at that new camera's actual focus
    *camera_transform = camera_transform.looking_at(camera_looking_at, Vec3::Y);
}
