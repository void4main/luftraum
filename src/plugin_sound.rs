use bevy::prelude::*;

#[derive(Event)]
pub struct PlaySoundEvent {
    pub sound_type: SoundType,
}

#[derive(Clone)]
pub enum SoundType {
    Attention,
}

#[derive(Resource)]
struct LuftraumSounds {
    fx_attention: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct SoundCooldown {
    pub(crate) timer: Timer,
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, sound_setup)
        .add_event::<PlaySoundEvent>()
        .insert_resource(SoundCooldown {
            timer: Timer::from_seconds(60.0, TimerMode::Once),
        })
        .add_systems(Update, (play_sound_on_event, update_cooldown));
}

fn sound_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Lade Sound-Dateien
    let sounds = LuftraumSounds {
        fx_attention: asset_server.load("sounds/luftraum_attention.ogg"),
    };
    commands.insert_resource(sounds);
}

fn update_cooldown(time: Res<Time>, mut cooldown: ResMut<SoundCooldown>) {
    cooldown.timer.tick(time.delta());
}

fn play_sound_on_event(
    mut commands: Commands,
    mut events: EventReader<PlaySoundEvent>,
    sounds: Res<LuftraumSounds>,
    mut cooldown: ResMut<SoundCooldown>,
) {
    for event in events.read() {
        let sound_handle = match event.sound_type {
            SoundType::Attention => sounds.fx_attention.clone(),
        };
        commands.spawn((AudioPlayer::new(sound_handle),));
        cooldown.timer.reset();
    }
}