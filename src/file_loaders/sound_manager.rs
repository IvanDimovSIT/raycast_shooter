use std::collections::{HashMap, HashSet};

use macroquad::audio::{
    load_sound, play_sound, play_sound_once, stop_sound, PlaySoundParams, Sound,
};

use crate::{constants::SOUND_PATH, model::SoundId};

const SOUND_PATHS: [(SoundId, &str); 6] = [
    (SoundId::PickUpKey, "pick_up_key.wav"),
    (SoundId::PlayerTakeDamage, "take_damage.wav"),
    (SoundId::Shooting, "shooting.wav"),
    (SoundId::ShotHit, "shot_hit.wav"),
    (SoundId::Lose, "lose.wav"),
    (SoundId::Escape, "escape.wav"),
];

pub struct SoundManager {
    sounds: HashMap<SoundId, Sound>,
    looped_sounds: HashSet<SoundId>,
}
impl SoundManager {
    async fn load_sounds(sounds: &mut HashMap<SoundId, Sound>, paths: &[(SoundId, &str)]) {
        for (id, path) in paths {
            let full_path = format!("{SOUND_PATH}{}", path);
            let sound = load_sound(&full_path).await;
            if let Ok(loaded_sound) = sound {
                sounds.insert(*id, loaded_sound);
            } else {
                println!("Failed to load sound: {}", path);
            }
        }
    }

    pub async fn load() -> Self {
        let mut sounds = HashMap::new();
        Self::load_sounds(&mut sounds, &SOUND_PATHS).await;
        Self {
            sounds,
            looped_sounds: HashSet::new(),
        }
    }

    pub fn play(&self, id: SoundId) {
        if self.sounds.contains_key(&id) {
            play_sound_once(&self.sounds[&id]);
        }
    }

    pub fn start_looped(&mut self, id: SoundId) {
        if !self.looped_sounds.contains(&id) && self.sounds.contains_key(&id) {
            play_sound(
                &self.sounds[&id],
                PlaySoundParams {
                    looped: true,
                    ..PlaySoundParams::default()
                },
            );
            self.looped_sounds.insert(id);
        }
    }

    pub fn stop_looped(&mut self, id: SoundId) {
        if self.looped_sounds.contains(&id) {
            self.looped_sounds.remove(&id);
            stop_sound(&self.sounds[&id]);
        }
    }

    pub fn stop_all(&mut self) {
        for sound in self.sounds.values() {
            stop_sound(sound);
        }
        self.looped_sounds.clear();
    }
}
