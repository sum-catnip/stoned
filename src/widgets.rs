use bevy::{color::palettes::css::RED, prelude::*};

pub fn w() -> impl Bundle {
    (
        BackgroundColor(RED.into()),
        Text::new("bennis"),
        Node {
            align_self: AlignSelf::Center,
            ..Default::default()
        },
    )
}
