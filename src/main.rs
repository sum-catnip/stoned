use avian3d::prelude::*;
use bevy::{
    dev_tools::picking_debug::{DebugPickingMode, DebugPickingPlugin},
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    image::ImageSamplerDescriptor,
    input::common_conditions::input_just_pressed,
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use bevy_egui::EguiPlugin;
use bevy_enhanced_input::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_skein::SkeinPlugin;

use crate::utils::ExampleUtilPlugin;

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
        .insert_resource(UiPickingSettings {
            require_markers: true,
        })
        .add_plugins((
            EnhancedInputPlugin,
            SkeinPlugin::default(),
            PhysicsPlugins::default(),
            PhysicsPickingPlugin,
            PhysicsDebugPlugin,
            DebugPickingPlugin,
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
        ))
        .insert_resource(DebugPickingMode::Normal)
        .add_plugins(ExampleUtilPlugin)
        .add_plugins(player::plugin)
        .add_systems(Startup, setup)
        //.add_systems(
        //    Update,
        //    (
        //        capture_cursor.run_if(input_just_pressed(MouseButton::Left)),
        //        release_cursor.run_if(input_just_pressed(KeyCode::Escape)),
        //    ),
        //)
        .init_resource::<Progress>()
        .run()
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((SceneRoot(assets.load("room.glb#Scene0")),));
}

fn capture_cursor(mut cursor: Single<&mut CursorOptions>) {
    cursor.grab_mode = CursorGrabMode::Locked;
    cursor.visible = false;
}

fn release_cursor(mut cursor: Single<&mut CursorOptions>) {
    cursor.visible = true;
    cursor.grab_mode = CursorGrabMode::None;
}

#[derive(Debug, PhysicsLayer, Default)]
pub enum CollisionLayer {
    #[default]
    Default,
    Player,
    Prop,
}

#[derive(Component, Reflect)]
#[require(Visibility)]
#[reflect(Component)]
#[type_path = "stoned"]
pub struct File;

#[derive(Resource, Default)]
pub struct Progress {
    pub files_collected: u64,
}

#[derive(Component, Reflect)]
#[require(ColliderConstructor::ConvexHullFromMesh)]
#[require(CollisionLayers::new(CollisionLayer::Prop, LayerMask::ALL))]
#[require(RigidBody::Dynamic)]
#[require(TransformInterpolation)]
#[require(ColliderDensity)]
#[reflect(Component, Default)]
#[component(on_add = onadd_prop)]
#[type_path = "stoned"]
pub struct PropMesh {
    density: f32,
}

impl Default for PropMesh {
    fn default() -> Self {
        Self { density: 1.0 }
    }
}

fn onadd_prop(mut w: DeferredWorld, ctx: HookContext) {
    debug!("prop added");
    let propmesh = w.get::<PropMesh>(ctx.entity).unwrap();
    let propmesh_density = propmesh.density;
    let mut density = w.get_mut::<ColliderDensity>(ctx.entity).unwrap();
    density.0 = propmesh_density;
}
