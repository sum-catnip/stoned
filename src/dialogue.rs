use bevy::prelude::*;
use bevy_seedling::sample::SamplePlayer;

use crate::{
    player::DisablePlayer,
    widgets::{DialogueTypewriter, dialogue_box, dismiss_ui},
};

pub fn intro(mut cmd: Commands, ass: Res<AssetServer>) {
    cmd.spawn((
        DialogueTypewriter::new(30.),
        dialogue_box("Orange Man", "I lost some files i mean no the files are strategically placed in the orfice. I am a money man i dont have time sorting through bureaucracy, the money today is in making stuff like fruit bowlia, like making it a really good place again. Anyways return the files to me and no peeking", ass.load("orangeman.gif")),
        SamplePlayer::new(ass.load("orange_talk.ogg")).looping(),
    ))
    .observe(dismiss_ui);
    cmd.trigger(DisablePlayer);
}

#[derive(Reflect, Clone, Copy)]
pub enum Dialogues {
    RelaxGuy,
    Romantic,
    Antichrist,
    Khole,
    Acid,
    Women,
}

pub struct StartDialogue(pub Dialogues);
impl Command for StartDialogue {
    fn apply(self, world: &mut World) -> () {
        match self.0 {
            Dialogues::RelaxGuy => world.run_system_cached(relax),
            Dialogues::Romantic => world.run_system_cached(romance),
            Dialogues::Antichrist => world.run_system_cached(antichrist),
            Dialogues::Khole => world.run_system_cached(khole),
            Dialogues::Acid => world.run_system_cached(acid),
            Dialogues::Women => world.run_system_cached(women),
        }
        .unwrap();
    }
}

fn relax(mut cmd: Commands, ass: Res<AssetServer>) {
    cmd.spawn((
        DialogueTypewriter::new(15.),
        dialogue_box("Orange Man", "Relax Guy", ass.load("orangeman.gif")),
        SamplePlayer::new(ass.load("orange_talk.ogg")).looping(),
    ))
    .observe(dismiss_ui);
    cmd.trigger(DisablePlayer);
}

fn romance(mut cmd: Commands, ass: Res<AssetServer>) {
    cmd.spawn((
        DialogueTypewriter::new(15.),
        dialogue_box(
            "Applestein",
            "ꖎᒷᒲᒲᒷ ⊣𝙹ᒷʖʖᒷꖎᓭ ℸ⍑ᔑℸ ∴ᒷᒷリ╎ᒷ",
            ass.load("applestein.gif"),
        ),
        SamplePlayer::new(ass.load("enchantment.ogg")),
    ))
    .observe(romance2);
    cmd.trigger(DisablePlayer);
}

fn romance2(on: On<Pointer<Click>>, mut cmd: Commands, ass: Res<AssetServer>) {
    cmd.entity(on.entity).despawn();
    cmd.spawn((
        DialogueTypewriter::new(15.),
        dialogue_box(
            "Orange Man",
            "Oh Applestein, show me what you did with Bubb - Oh you're back already, what do you want? Get back to work",
            ass.load("orangeman.gif"),
        ),
        SamplePlayer::new(ass.load("orange_talk.ogg")),
    ))
    .observe(dismiss_ui);
}

fn antichrist(mut cmd: Commands, ass: Res<AssetServer>) {
    cmd.spawn((
        DialogueTypewriter::new(15.),
        dialogue_box(
            "Orange Man",
            "Please stop talking to me about the Antichrist",
            ass.load("orangeman.gif"),
        ),
        SamplePlayer::new(ass.load("orange_talk.ogg")).looping(),
    ))
    .observe(dismiss_ui);
    cmd.trigger(DisablePlayer);
}

fn khole(mut cmd: Commands, ass: Res<AssetServer>) {
    cmd.spawn((
        DialogueTypewriter::new(15.),
        dialogue_box(
            "Orange Man",
            "AAAAAAAAAAAAAAAAAND im in a k-hole",
            ass.load("orangeman.gif"),
        ),
        SamplePlayer::new(ass.load("orange_talk.ogg")).looping(),
    ))
    .observe(dismiss_ui);
    cmd.trigger(DisablePlayer);
}

fn acid(mut cmd: Commands, ass: Res<AssetServer>) {
    cmd.spawn((
        DialogueTypewriter::new(25.),
        dialogue_box(
            "Applestein",
            "ᓵᔑリ ╎ ⊣ᒷℸ ᔑ ℸᔑ̇/ ∷ᒷℸ⚍∷リ 𝙹リ ℸ⍑ᒷ ᓭ⚍ꖎ⎓⚍∷╎ᓵ ᔑᓵ╎↸ ╎ ⚍ᓭᒷ↸ ⎓𝙹∷ ↸╎ᓭᓭ𝙹ꖎ⍊╎リ⊣ ℸ⍑ᒷ ᓵ⍑╎ꖎ↸∷ᒷリ ∴ᒷ ᔑʖ⚍ᓭᒷ↸?",
            ass.load("applestein.gif"),
        ),
        SamplePlayer::new(ass.load("enchantment.ogg")).looping(),
    ))
    .observe(dismiss_ui);
    cmd.trigger(DisablePlayer);
}

fn women(mut cmd: Commands, ass: Res<AssetServer>) {
    cmd.spawn((
        DialogueTypewriter::new(30.),
        dialogue_box(
            "Orange Man",
            "Women i have respect for woman, maybe the most respect anyone has for them. Friends of mine are women believe it or not i have lots of friends old ones young ones it doesnt matter its all the same to me",
            ass.load("orangeman.gif"),
        ),
        SamplePlayer::new(ass.load("orange_talk.ogg")).looping(),
    ))
    .observe(dismiss_ui);
    cmd.trigger(DisablePlayer);
}
