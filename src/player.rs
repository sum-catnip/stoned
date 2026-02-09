use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_ahoy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::utils::StableGround;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(AhoyPlugins::default())
        .add_input_context::<PlayerInput>()
        .add_observer(spawn_player);
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Default)]
pub(crate) struct PlayerInput;

fn spawn_player(_: On<Add, Player>, mut cmd: Commands) {
    debug!("spawning player");
    let player = cmd
        .spawn((
            // Add the character controller configuration. We'll use the default settings for now.
            CharacterController::default(),
            // The KCC currently behaves best when using a cylinder
            Collider::cylinder(0.4, 1.8),
            StableGround::default(),
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
                        // tweak mouse and right stick sensitivity
                        // in Scale::splat values
                        Spawn((Binding::mouse_motion(), Scale::splat(0.07))),
                        Axial::right_stick().with((Scale::splat(4.0), DeadZone::default())),
                    ))
                ),
            ]),
        ))
        .id();

    // Spawn the player camera
    cmd.spawn((
        Camera3d::default(),
        // Enable the optional builtin camera controller
        CharacterControllerCameraOf::new(player),
    ));
}
