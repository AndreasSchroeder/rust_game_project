// Module for the SoundHandler
use ears::{Sound, AudioController};
use std::collections::HashMap;
use find_folder;

/// Struct for saving and playing soundfiles.
/// map: A HashMap with name of sound-file as key and Sound from ears-libary as Sound
pub struct SoundHandler {
    pub map: HashMap<&'static str, Sound>,
}

impl SoundHandler {
    /// Fills the Soundhandler with Sounds given in const SOUNDS.
    /// The Filename is the key and the ears::Sound is the value
    pub fn fill() -> Self {
        let mut map: HashMap<&str, Sound> = HashMap::new();
        // iterate over all soundfiles
        for path in SOUNDS {
            // Find the Sound-folder in directory
            if let Ok(sound_path) = find_folder::Search::ParentsThenKids(2, 2)
                .for_folder("Sounds") {
                // Create path with found folder-path and sound name
                let s_path = sound_path.join(path);
                // Create String from path
                if let Some(sound_string) = s_path.to_str() {
                    // Open path and create Sound
                    if let Some(sound) = Sound::new(sound_string) {
                        // insert sound
                        map.insert(path, sound);

                    } else {
                        println!("Unable to build Sound-file: {}", path);
                    }
                    // Insert Sound in HashMap
                } else {
                    println!("Unable to build Path to String: {}", path);
                }
            } else {
                println!("File: {} not found in: {}", path, "Sounds")
            }
        }
        SoundHandler { map: map }
    }

    /// plays a Sound, if in SoundMap. Else prints sound which was not found
    pub fn play(&mut self, sound: &str) {
        if let Some(plays) = self.map.get_mut(sound) {
            plays.play();
        } else {
            println!("File: {} not found in SoundMap", sound);
        }
    }
}

/// The Sounds
const SOUNDS: &'static [&'static str] = &["Chicken.ogg",
                                          "Dagger.ogg",
                                          "Dead.ogg",
                                          "Item.ogg",
                                          "Spear.ogg",
                                          "Sword.ogg",
                                          "test.ogg",
                                          "Welcome.ogg",
                                          "Background.ogg",
                                          "Wilhelm.ogg"];
