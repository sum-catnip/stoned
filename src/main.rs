use avian3d::prelude::*;
use bevy::{
    image::ImageSamplerDescriptor,
    input::common_conditions::input_just_pressed,
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use bevy_enhanced_input::prelude::*;
use bevy_skein::SkeinPlugin;

use crate::utils::{ExampleUtilPlugin, StableGround};

mod player;
mod utils;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: ImageSamplerDescriptor {
                anisotropy_clamp: 1,
                ..ImageSamplerDescriptor::linear()
            },
        }))
        .add_plugins((
            EnhancedInputPlugin,
            SkeinPlugin::default(),
            PhysicsPlugins::default(),
        ))
        .add_plugins(ExampleUtilPlugin)
        .add_plugins(player::plugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                capture_cursor.run_if(input_just_pressed(MouseButton::Left)),
                release_cursor.run_if(input_just_pressed(KeyCode::Escape)),
            ),
        )
        .run()
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    debug!("hi");
    // Spawn a directional light
    commands.spawn((
        Transform::from_xyz(0.0, 1.0, 0.0).looking_at(vec3(1.0, -2.0, -2.0), Vec3::Y),
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
    ));

    // Spawn the level. This can be done in whatever way you prefer: spawn individual colliders, load a scene, use Skein, use bevy_trenchbroom, etc.
    // Ahoy will deal with it all.
    // Here we load a glTF file and create a convex hull collider for each mesh.
    commands.spawn((
        SceneRoot(assets.load("room.glb#Scene0")),
        //RigidBody::Static,
        //ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
    ));
}

fn capture_cursor(mut cursor: Single<&mut CursorOptions>) {
    cursor.grab_mode = CursorGrabMode::Locked;
    cursor.visible = false;
}

fn release_cursor(mut cursor: Single<&mut CursorOptions>) {
    cursor.visible = true;
    cursor.grab_mode = CursorGrabMode::None;
}
