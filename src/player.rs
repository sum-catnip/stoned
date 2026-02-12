use avian_pickup::{
    Holding,
    actor::AvianPickupActorState,
    input::{AvianPickupAction, AvianPickupInput},
};
use avian3d::prelude::*;
use bevy::{
    anti_alias::fxaa::Fxaa,
    core_pipeline::{prepass::DepthPrepass, tonemapping::Tonemapping},
    ecs::entity_disabling::Disabled,
    post_process::{bloom::Bloom, dof::DepthOfField, effect_stack::ChromaticAberration},
    prelude::*,
    render::view::Hdr,
};
use bevy_ahoy::{PickupHoldConfig, PickupPullConfig, prelude::*};
use bevy_enhanced_input::prelude::Press;
use bevy_enhanced_input::prelude::*;

use crate::{CollisionLayer, File, FileCollected, Progress, k::K};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(AhoyPlugins::default())
        .add_systems(Update, check_page_collect)
        .add_input_context::<PlayerInput>()
        .init_resource::<PlayerRes>()
        .add_observer(on_enable)
        .add_observer(on_disable)
        .add_observer(spawn_player);
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Default)]
pub(crate) struct PlayerInput;

#[derive(Resource, Default)]
pub struct PlayerRes {
    pub player: Option<Entity>,
    pub cam: Option<Entity>,
}

#[derive(Event)]
pub struct DisablePlayer;

#[derive(Event)]
pub struct EnablePlayer;

fn spawn_player(
    t: On<Add, Player>,
    trans: Query<&Transform>,
    mut cmd: Commands,
    mut playerres: ResMut<PlayerRes>,
) {
    debug!("spawning player");
    let player = cmd
        .spawn((
            *trans.get(t.entity).unwrap(),
            Pickable::IGNORE,
            Name::new("player controller"),
            // Add the character controller configuration. We'll use the default settings for now.
            CharacterController::default(),
            CollisionLayers::new(CollisionLayer::Player, LayerMask::ALL),
            Mass(90.0),
            // The KCC currently behaves best when using a cylinder
            Collider::cylinder(0.4, 1.8),
            //StableGround::default(),
            // Configure inputs. The actions `Movement`, `Jump`, etc. are provided by Ahoy, you just need to bind them.
            PlayerInput,
            actions!(PlayerInput[
                (
                    Action::<Movement>::new(),
                    // Normalize the input vector
                    DeadZone::default(),
                    Bindings::spawn((
                        Cardinal::wasd_keys(),
                        Axial::left_stick()
                    ))
                ),
                (
                    Action::<Jump>::new(),
                    bindings![KeyCode::Space,  GamepadButton::South],
                ),
                (
                    Action::<Crouch>::new(),
                    bindings![KeyCode::ControlLeft, GamepadButton::LeftTrigger2],
                ),
                (
                    Action::<RotateCamera>::new(),
                    Bindings::spawn((
                        Spawn((Binding::mouse_motion(), Scale::splat(0.07))),
                        Axial::right_stick().with((Scale::splat(4.0), DeadZone::default())),
                    ))
                ),
                (
                    Action::<PullObject>::new(),
                    ActionSettings { consume_input: true, ..default() },
                    Press::default(),
                    bindings![KeyCode::KeyE],
                ),
                (
                    Action::<ThrowObject>::new(),
                    ActionSettings { consume_input: true, ..default() },
                    Press::default(),
                    bindings![KeyCode::KeyQ],
                ),
            ]),
        ))
        .id();

    playerres.player = Some(player);

    // Spawn the player camera
    let playercam = cmd
        .spawn((
            Name::new("player cam"),
            K::default(),
            PickupConfig {
                prop_filter: SpatialQueryFilter::from_mask(CollisionLayer::Prop),
                actor_filter: SpatialQueryFilter::from_mask(CollisionLayer::Player),
                obstacle_filter: SpatialQueryFilter::from_mask(CollisionLayer::Default),
                hold: PickupHoldConfig {
                    preferred_distance: 0.9,
                    linear_velocity_easing: 0.3,
                    ..default()
                },
                pull: PickupPullConfig {
                    max_prop_mass: 1000.0,
                    ..default()
                },
                ..Default::default()
            },
            UiPickingCamera,
            IsDefaultUiCamera,
            (
                //Msaa::Sample8,
                // has to be off for wasm build
                Msaa::Off,
                Fxaa::default(),
                Bloom::NATURAL,
                Hdr,
                DepthPrepass,
                DepthOfField {
                    focal_distance: 5.,
                    sensor_height: 0.001,
                    ..Default::default()
                },
                ChromaticAberration::default(),
                Tonemapping::BlenderFilmic,
            ),
            Camera3d::default(),
            // Enable the optional builtin camera controller
            CharacterControllerCameraOf::new(player),
        ))
        .id();

    playerres.cam = Some(playercam);
}

fn check_page_collect(
    mut cmd: Commands,
    mut actor_state: Single<(Entity, &mut AvianPickupActorState)>,
    files: Query<Entity, With<File>>,
) {
    let AvianPickupActorState::Holding(e) = actor_state.1.as_ref() else {
        return;
    };

    if !files.contains(*e) {
        return;
    }

    cmd.trigger(FileCollected { file: *e });
    *actor_state.1.as_mut() = AvianPickupActorState::Idle;
    cmd.entity(actor_state.0).remove::<Holding>();
}

fn on_enable(_: On<EnablePlayer>, mut cmd: Commands, player: Res<PlayerRes>) {
    if let Some(e) = player.player {
        cmd.entity(e).insert(ContextActivity::<PlayerInput>::ACTIVE);
    }
}

fn on_disable(_: On<DisablePlayer>, mut cmd: Commands, player: Res<PlayerRes>) {
    if let Some(e) = player.player {
        cmd.entity(e)
            .insert(ContextActivity::<PlayerInput>::INACTIVE);
    }
}
