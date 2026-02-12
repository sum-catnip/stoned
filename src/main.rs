use std::time::Duration;
mod blob;
mod dialogue;
mod k;
mod player;
mod utils;
mod widgets;

use avian3d::prelude::*;
use bevy::{
    dev_tools::picking_debug::{DebugPickingMode, DebugPickingPlugin},
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    image::ImageSamplerDescriptor,
    input::common_conditions::input_just_pressed,
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use bevy_easy_gif::GifPlugin;
use bevy_egui::EguiPlugin;
use bevy_enhanced_input::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_seedling::{SeedlingPlugin, sample::SamplePlayer};
use bevy_skein::SkeinPlugin;

use crate::{
    blob::Blob,
    dialogue::{Dialogues, StartDialogue, intro},
    player::DisablePlayer,
    utils::ExampleUtilPlugin,
    widgets::{FadeIn, credits_screen, l, timer},
};

const FILES: u32 = 1;

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(ImagePlugin {
        default_sampler: ImageSamplerDescriptor {
            anisotropy_clamp: 1,
            ..ImageSamplerDescriptor::linear()
        },
    }))
    .insert_resource(UiPickingSettings {
        require_markers: true,
    })
    .add_plugins((
        GifPlugin,
        SeedlingPlugin::default(),
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
    .add_plugins((player::plugin, widgets::plugin, blob::plugin, k::plugin))
    .add_systems(Startup, setup)
    .add_systems(Update, tick_progress)
    .add_systems(
        Update,
        (
            capture_cursor.run_if(input_just_pressed(MouseButton::Left)),
            release_cursor.run_if(input_just_pressed(KeyCode::Escape)),
        ),
    )
    .insert_resource(Progress {
        files_collected: 0,
        timer: Timer::new(Duration::from_mins(5), TimerMode::Once),
    })
    .init_resource::<Progress>()
    .add_observer(on_file_collected)
    .add_observer(on_w)
    .add_observer(on_l);

    app.run()
}

fn setup(mut cmd: Commands, assets: Res<AssetServer>) {
    cmd.spawn((SceneRoot(assets.load("room.glb#Scene0")),));
    cmd.run_system_cached(intro);
    cmd.spawn(timer());
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
pub struct File {
    pub file: String,
    pub sound: String,
    pub dialogue: Dialogues,
}

#[derive(Resource, Default)]
pub struct Progress {
    pub files_collected: u32,
    pub timer: Timer,
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
pub struct DynamicProp {
    density: f32,
}

impl Default for DynamicProp {
    fn default() -> Self {
        Self { density: 1.0 }
    }
}

fn onadd_prop(mut w: DeferredWorld, ctx: HookContext) {
    debug!("prop added");
    let propmesh = w.get::<DynamicProp>(ctx.entity).unwrap();
    let propmesh_density = propmesh.density;
    let mut density = w.get_mut::<ColliderDensity>(ctx.entity).unwrap();
    density.0 = propmesh_density;
}

#[derive(Component, Reflect)]
#[require(ColliderConstructor::TrimeshFromMesh)]
#[require(CollisionLayers::new(CollisionLayer::Default, LayerMask::ALL))]
#[require(ColliderDensity(3000.))]
#[require(RigidBody::Static)]
#[reflect(Component)]
#[type_path = "stoned"]
pub struct StaticProp;

#[derive(EntityEvent)]
pub struct FileCollected {
    #[event_target]
    pub file: Entity,
}

fn on_file_collected(
    on: On<FileCollected>,
    mut cmd: Commands,
    mut prog: ResMut<Progress>,
    files: Query<&File>,
    ass: Res<AssetServer>,
) {
    debug!("file collected, yay");
    let file = files.get(on.file).unwrap();
    cmd.spawn(SamplePlayer::new(ass.load(&file.sound)));
    let _: Handle<Blob> = ass.load(&file.file);
    prog.files_collected += 1;
    cmd.queue(StartDialogue(file.dialogue));
    cmd.entity(on.file).despawn();
    if prog.files_collected == FILES {
        cmd.trigger(W);
    }
}

fn tick_progress(mut prog: ResMut<Progress>, time: Res<Time>, mut cmd: Commands) {
    if prog.timer.tick(time.delta()).just_finished() {
        cmd.trigger(BigL);
    }
}

#[derive(Event)]
pub struct W;

#[derive(Event)]
pub struct BigL;

fn on_w(_: On<W>, mut cmd: Commands, ass: Res<AssetServer>) {
    debug!("W");
    cmd.trigger(DisablePlayer);
    cmd.spawn(SamplePlayer::new(ass.load("siren.ogg")).looping());
    cmd.spawn((
        l(ass.load("souls_font.ttf"), "Du Wurdest Verepp-elt"),
        FadeIn::new(1.5),
    ))
    .observe(trigger_credits);
}

fn on_l(_: On<BigL>, mut cmd: Commands, ass: Res<AssetServer>) {
    debug!("L");
    cmd.trigger(DisablePlayer);
    cmd.spawn((
        l(ass.load("souls_font.ttf"), "Du Wurdest Gestein-Rolled"),
        FadeIn::new(1.5),
    ))
    .observe(trigger_credits);
}

fn trigger_credits(on: On<Pointer<Click>>, mut cmd: Commands, prog: Res<Progress>) {
    let time = prog.timer.elapsed().as_secs_f32();
    cmd.entity(on.entity).despawn();
    cmd.spawn(credits_screen(time));
}
