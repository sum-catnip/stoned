use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, animate_fadein);
}

#[derive(Component)]
pub struct YouDiedScreen;

#[derive(Component)]
pub struct YouDiedText;

const YOU_DIED_RED: Color = Color::srgba(0.54, 0.07, 0.07, 1.0); // #8a1212

const RULE_COLOR: Color = Color::srgba(0.35, 0.04, 0.04, 0.6);

const OVERLAY_BG: Color = Color::srgba(0.0, 0.0, 0.0, 0.72);

pub fn l(font: Handle<Font>) -> impl Bundle {
    (
        YouDiedScreen,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(0.0),
            ..default()
        },
        GlobalZIndex(999),
        BackgroundColor(OVERLAY_BG),
        BoxShadow(vec![ShadowStyle {
            color: Color::srgba(0.0, 0.0, 0.0, 0.85),
            x_offset: Val::Px(0.0),
            y_offset: Val::Px(0.0),
            spread_radius: Val::Percent(30.0),
            blur_radius: Val::Px(180.0),
        }]),
        children![you_died_rule(), you_died_text(font), you_died_rule(),],
    )
}

fn you_died_text(font: Handle<Font>) -> impl Bundle {
    (
        YouDiedText,
        Node {
            margin: UiRect::axes(Val::Px(0.0), Val::Px(18.0)),
            ..default()
        },
        Text::new("Du Wurdest Gestein-rolled"),
        TextFont {
            font_size: 96.0,
            font,
            ..default()
        },
        TextColor(YOU_DIED_RED),
        BoxShadow(vec![ShadowStyle {
            color: Color::srgba(0.54, 0.07, 0.07, 0.35),
            x_offset: Val::Px(0.0),
            y_offset: Val::Px(0.0),
            spread_radius: Val::Px(4.0),
            blur_radius: Val::Px(40.0),
        }]),
    )
}

fn you_died_rule() -> impl Bundle {
    (
        Node {
            width: Val::Px(420.0),
            height: Val::Px(2.0),
            ..default()
        },
        BackgroundColor(RULE_COLOR),
    )
}

#[derive(Component)]
pub struct FadeIn {
    /// Total duration of the fade in seconds.
    pub duration: f32,
    /// Elapsed time so far.
    pub elapsed: f32,
}

impl FadeIn {
    pub fn new(duration: f32) -> Self {
        Self {
            duration,
            elapsed: 0.0,
        }
    }
}

pub fn animate_fadein(
    time: Res<Time>,
    mut query: Query<(&mut FadeIn, &mut BackgroundColor)>,
    mut text_query: Query<(&mut TextColor, &ChildOf), With<YouDiedText>>,
) {
    for (mut fade, mut bg) in &mut query {
        fade.elapsed = (fade.elapsed + time.delta_secs()).min(fade.duration);
        let t = ease_out_cubic(fade.elapsed / fade.duration);

        // Fade the overlay background alpha
        let mut c = OVERLAY_BG;
        c.set_alpha(c.alpha() * t);
        *bg = BackgroundColor(c);

        // Fade the text colour for any YouDiedText children
        for (mut text_color, _) in &mut text_query {
            let mut tc = YOU_DIED_RED;
            tc.set_alpha(t);
            text_color.0 = tc;
        }
    }
}

fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}
