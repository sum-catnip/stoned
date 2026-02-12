use bevy::prelude::*;
use bevy_easy_gif::{GifAsset, GifNode};

use crate::{Progress, player::EnablePlayer};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (animate_fadein, update_timer, dialogue_typewriter_system),
    );
}

#[derive(Component)]
pub struct SoulsSceen;

#[derive(Component)]
pub struct SoulsText;

#[derive(Component)]
pub struct CreditsScreen;

#[derive(Component)]
pub struct DialogueOverlay;

#[derive(Component)]
pub struct DialoguePanel;

#[derive(Component)]
pub struct DialogueText;

#[derive(Component)]
pub struct DialogueSpeakerName;

#[derive(Component)]
pub struct DialoguePortrait;

const SOULS_RED: Color = Color::srgba(0.54, 0.07, 0.07, 1.0); // #8a1212

const BAR_COLOR: Color = Color::srgba(0.35, 0.04, 0.04, 0.6);

const OVERLAY_BG: Color = Color::srgba(0.0, 0.0, 0.0, 0.72);

const BG: Color = Color::srgba(0.04, 0.04, 0.06, 0.95);
const HEADING_COLOR: Color = Color::srgb(0.93, 0.79, 0.39);
const NAME_COLOR: Color = Color::srgb(0.92, 0.92, 0.94);
const ROLE_COLOR: Color = Color::srgb(0.55, 0.55, 0.60);
const TIMER_COLOR: Color = Color::srgb(0.70, 0.82, 0.65);
const HINT_COLOR: Color = Color::srgb(0.45, 0.45, 0.50);
const DIVIDER_COLOR: Color = Color::srgba(0.93, 0.79, 0.39, 0.25);

const BACKDROP_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.40);

const PANEL_BG: Color = Color::srgba(0.08, 0.07, 0.10, 0.92);
const PANEL_BORDER: Color = Color::srgba(0.55, 0.48, 0.30, 0.70);
const PANEL_SHADOW: Color = Color::srgba(0.0, 0.0, 0.0, 0.60);

const SPEAKER_COLOR: Color = Color::srgb(0.93, 0.79, 0.39);
const BODY_COLOR: Color = Color::srgb(0.88, 0.88, 0.90);

const PORTRAIT_SIZE: f32 = 96.0;
const PORTRAIT_BORDER: f32 = 3.0;

#[derive(Component)]
pub struct TimerUi;

pub fn timer() -> impl Bundle {
    (
        Node {
            margin: UiRect::all(Val::Px(100.)),
            ..Default::default()
        },
        Text::default(),
        TextFont {
            font_size: 42.,
            ..Default::default()
        },
        TimerUi,
    )
}

pub fn dismiss_ui(on: On<Pointer<Click>>, mut cmd: Commands) {
    cmd.trigger(EnablePlayer);
    cmd.entity(on.entity).despawn();
}

pub fn dialogue_box(
    speaker: impl Into<String>,
    body: impl Into<String>,
    portrait: Handle<GifAsset>,
) -> impl Bundle {
    (
        DialogueOverlay,
        Pickable {
            should_block_lower: true,
            is_hoverable: true,
        },
        // Full-screen transparent backdrop
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::Center,
            padding: UiRect::new(Val::Px(32.0), Val::Px(32.0), Val::Px(0.0), Val::Px(40.0)),
            ..default()
        },
        GlobalZIndex(900),
        BackgroundColor(BACKDROP_COLOR),
        children![dialogue_panel(speaker, body, portrait),],
    )
}

pub fn credits_screen(elapsed_secs: f32) -> impl Bundle {
    let mins = (elapsed_secs / 60.0).floor() as u32;
    let secs = (elapsed_secs % 60.0).floor() as u32;
    let time_str = format!(
        "Finished in {:02}:{:02}. Refresh page to play again.",
        mins, secs
    );

    (
        CreditsScreen,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(28.0),
            padding: UiRect::all(Val::Px(40.0)),
            ..default()
        },
        GlobalZIndex(1000),
        BackgroundColor(BG),
        children![
            timer_hint(time_str),
            spacer(18.0),
            section_heading("CREDITS"),
            divider(),
            credit_person("FunthomTomate", "3D Art & Room Design"),
            credit_person("Malenia", "2D Art"),
            credit_person("catnip", "Programming & Concept"),
            divider(),
            section_heading("ASSETS"),
            asset_credit("\"Dark Souls Serif Font\"", "dafontfree.co"),
            asset_credit("\"Airhorn Sound Effect\"", "DRAGON-STUDIO"),
            asset_credit("\"Agenda\"", "Antifa"),
            spacer(12.0),
            footer_text("Made with <3 with the Bevy gameengine"),
        ],
    )
}

pub fn l(font: Handle<Font>, text: impl Into<String>) -> impl Bundle {
    (
        SoulsSceen,
        Pickable {
            is_hoverable: true,
            should_block_lower: true,
        },
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
        children![
            souls_bar(),
            souls_text(font, text),
            souls_bar(),
            click_to_continue()
        ],
    )
}

fn click_to_continue() -> impl Bundle {
    (
        Text::new("click to continue"),
        Node {
            align_content: AlignContent::Center,
            ..Default::default()
        },
    )
}

fn souls_text(font: Handle<Font>, text: impl Into<String>) -> impl Bundle {
    (
        SoulsText,
        Node {
            margin: UiRect::axes(Val::Px(0.0), Val::Px(18.0)),
            ..default()
        },
        Text::new(text),
        TextFont {
            font_size: 96.0,
            font,
            ..default()
        },
        TextColor(SOULS_RED),
        BoxShadow(vec![ShadowStyle {
            color: Color::srgba(0.54, 0.07, 0.07, 0.35),
            x_offset: Val::Px(0.0),
            y_offset: Val::Px(0.0),
            spread_radius: Val::Px(4.0),
            blur_radius: Val::Px(40.0),
        }]),
    )
}

fn souls_bar() -> impl Bundle {
    (
        Node {
            width: Val::Px(420.0),
            height: Val::Px(2.0),
            ..default()
        },
        BackgroundColor(BAR_COLOR),
    )
}

#[derive(Component)]
pub struct FadeIn {
    pub duration: f32,
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
    mut text_query: Query<(&mut TextColor, &ChildOf), With<SoulsText>>,
) {
    for (mut fade, mut bg) in &mut query {
        fade.elapsed = (fade.elapsed + time.delta_secs()).min(fade.duration);
        let t = ease_out_cubic(fade.elapsed / fade.duration);

        // Fade the overlay background alpha
        let mut c = OVERLAY_BG;
        c.set_alpha(c.alpha() * t);
        *bg = BackgroundColor(c);

        // Fade the text colour for any children
        for (mut text_color, _) in &mut text_query {
            let mut tc = SOULS_RED;
            tc.set_alpha(t);
            text_color.0 = tc;
        }
    }
}

fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}

fn timer_hint(text: impl Into<String>) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font_size: 22.0,
            ..default()
        },
        TextColor(TIMER_COLOR),
        Node {
            margin: UiRect::bottom(Val::Px(4.0)),
            ..default()
        },
    )
}

fn section_heading(label: impl Into<String>) -> impl Bundle {
    (
        Text::new(label),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(HEADING_COLOR),
        Node {
            margin: UiRect::bottom(Val::Px(4.0)),
            ..default()
        },
    )
}

fn credit_person(name: impl Into<String>, role: impl Into<String>) -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(2.0),
            margin: UiRect::axes(Val::Px(0.0), Val::Px(6.0)),
            ..default()
        },
        children![
            (
                Text::new(name),
                TextFont {
                    font_size: 26.0,
                    ..default()
                },
                TextColor(NAME_COLOR),
            ),
            (
                Text::new(role),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(ROLE_COLOR),
            ),
        ],
    )
}

fn asset_credit(asset_name: impl Into<String>, attribution: impl Into<String>) -> impl Bundle {
    let line = format!("{} — {}", asset_name.into(), attribution.into());
    (
        Text::new(line),
        TextFont {
            font_size: 17.0,
            ..default()
        },
        TextColor(ROLE_COLOR),
        Node {
            margin: UiRect::axes(Val::Px(0.0), Val::Px(2.0)),
            ..default()
        },
    )
}

fn divider() -> impl Bundle {
    (
        Node {
            width: Val::Px(320.0),
            height: Val::Px(1.0),
            margin: UiRect::axes(Val::Px(0.0), Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(DIVIDER_COLOR),
    )
}

fn spacer(height: f32) -> impl Bundle {
    Node {
        height: Val::Px(height),
        ..default()
    }
}

fn footer_text(text: impl Into<String>) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(HINT_COLOR),
    )
}

fn update_timer(mut label: Single<&mut Text, With<TimerUi>>, time: Res<Progress>) {
    let duration = time.timer.remaining();
    label.0 = format!("{}", duration.as_secs());
}

fn dialogue_panel(
    speaker: impl Into<String>,
    body: impl Into<String>,
    portrait: Handle<GifAsset>,
) -> impl Bundle {
    (
        DialoguePanel,
        Node {
            width: Val::Percent(100.0),
            max_width: Val::Px(760.0),
            padding: UiRect::all(Val::Px(18.0)),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::FlexStart,
            column_gap: Val::Px(18.0),
            border_radius: BorderRadius::all(Val::Px(6.0)),
            ..default()
        },
        BackgroundColor(PANEL_BG),
        BorderColor::all(PANEL_BORDER),
        BoxShadow(vec![ShadowStyle {
            color: PANEL_SHADOW,
            x_offset: Val::Px(0.0),
            y_offset: Val::Px(4.0),
            spread_radius: Val::Px(2.0),
            blur_radius: Val::Px(24.0),
        }]),
        children![portrait_frame(portrait), text_column(speaker, body),],
    )
}

fn portrait_frame(image: Handle<GifAsset>) -> impl Bundle {
    (
        DialoguePortrait,
        Node {
            //width: Val::Px(PORTRAIT_SIZE),
            //height: Val::Px(PORTRAIT_SIZE),
            min_width: Val::Px(PORTRAIT_SIZE),
            border: UiRect::all(Val::Px(PORTRAIT_BORDER)),
            border_radius: BorderRadius::all(Val::Px(4.0)),
            ..default()
        },
        BorderColor::all(PANEL_BORDER),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
        GifNode { handle: image },
    )
}

fn text_column(speaker: impl Into<String>, body: impl Into<String>) -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(6.0),
            flex_grow: 1.0,
            ..default()
        },
        children![
            // Speaker
            (
                DialogueSpeakerName,
                Text::new(speaker),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(SPEAKER_COLOR),
            ),
            // Body
            (
                DialogueText,
                Text::new(body),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(BODY_COLOR),
                Node {
                    margin: UiRect::top(Val::Px(2.0)),
                    ..default()
                },
            ),
            (
                Text::new("[left-click] to continue"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(HINT_COLOR),
                Node {
                    margin: UiRect::top(Val::Px(10.0)),
                    ..default()
                },
            ),
        ],
    )
}

#[derive(Component)]
pub struct DialogueTypewriter {
    pub chars_per_sec: f32,
    pub full_text: Option<String>,
    pub revealed: f32,
}

impl DialogueTypewriter {
    pub fn new(chars_per_sec: f32) -> Self {
        Self {
            chars_per_sec,
            full_text: None,
            revealed: 0.0,
        }
    }
}

pub fn dialogue_typewriter_system(
    time: Res<Time>,
    mut tw_query: Query<(Entity, &mut DialogueTypewriter)>,
    children_query: Query<&Children>,
    mut text_query: Query<&mut Text, With<DialogueText>>,
) {
    for (overlay_entity, mut tw) in &mut tw_query {
        for descendant in children_query.iter_descendants(overlay_entity) {
            if let Ok(mut text) = text_query.get_mut(descendant) {
                let full = tw.full_text.get_or_insert_with(|| text.0.clone()).clone();
                let total_chars = full.chars().count() as f32;
                tw.revealed = (tw.revealed + tw.chars_per_sec * time.delta_secs()).min(total_chars);
                let visible: String = full.chars().take(tw.revealed as usize).collect();
                text.0 = visible;
            }
        }
    }
}
