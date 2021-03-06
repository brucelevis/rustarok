use crate::audio::sound_sys::{SoundChunkStore, SoundId, SoundSystem, DUMMY_SOUND_ID};
use crate::grf::asset_loader::GrfEntryLoader;

pub struct Sounds {
    pub attack: SoundId,
    pub arrow_hit: SoundId,
    pub arrow_attack: SoundId,
    pub gun_attack: SoundId,
    pub stun: SoundId,
    pub heal: SoundId,
    pub firewall: SoundId,
}

impl Sounds {
    pub fn new_for_test() -> Sounds {
        Sounds {
            attack: DUMMY_SOUND_ID,
            arrow_hit: DUMMY_SOUND_ID,
            arrow_attack: DUMMY_SOUND_ID,
            gun_attack: DUMMY_SOUND_ID,
            stun: DUMMY_SOUND_ID,
            heal: DUMMY_SOUND_ID,
            firewall: DUMMY_SOUND_ID,
        }
    }
}

pub fn init_audio_and_load_sounds(
    sdl_context: &sdl2::Sdl,
    asset_loader: &GrfEntryLoader,
) -> (Option<SoundSystem>, Sounds) {
    return if let Ok(sdl_audio) = sdl_context.audio() {
        init_audio();
        let mut sound_store = SoundChunkStore::new();
        let sounds = load_sounds(&asset_loader, &mut sound_store);
        let sound_system = SoundSystem::new(sdl_audio, sound_store);
        (Some(sound_system), sounds)
    } else {
        (None, Sounds::new_for_test())
    };
}

fn init_audio() {
    let frequency = sdl2::mixer::DEFAULT_FREQUENCY;
    let format = sdl2::mixer::DEFAULT_FORMAT; // signed 16 bit samples, in little-endian byte order
    let channels = sdl2::mixer::DEFAULT_CHANNELS; // Stereo
    let chunk_size = 1_024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size).expect("");
    let _mixer_context = sdl2::mixer::init(
        sdl2::mixer::InitFlag::MP3
            | sdl2::mixer::InitFlag::FLAC
            | sdl2::mixer::InitFlag::MOD
            | sdl2::mixer::InitFlag::OGG,
    )
    .expect("");
    sdl2::mixer::allocate_channels(4);
    sdl2::mixer::Channel::all().set_volume(16);
}

fn load_sounds(asset_loader: &GrfEntryLoader, chunk_store: &mut SoundChunkStore) -> Sounds {
    // TODO: use dummy sound instead of unwrap
    let sounds = Sounds {
        attack: chunk_store
            .load_wav("data\\wav\\_novice_attack.wav", asset_loader)
            .unwrap(),
        arrow_hit: chunk_store
            .load_wav("data\\wav\\_archer_hit.wav", asset_loader)
            .unwrap(),
        arrow_attack: chunk_store
            .load_wav("data\\wav\\attack_bow.wav", asset_loader)
            .unwrap(),
        gun_attack: chunk_store
            .load_wav("data\\wav\\gunfire.wav", asset_loader)
            .unwrap(),
        stun: chunk_store
            .load_wav("data\\wav\\_stun.wav", asset_loader)
            .unwrap(),
        heal: chunk_store
            .load_wav("data\\wav\\_heal_effect.wav", asset_loader)
            .unwrap(),
        firewall: chunk_store
            .load_wav("data\\wav\\effect\\ef_firewall.wav", asset_loader)
            .unwrap(),
    };
    return sounds;
}
