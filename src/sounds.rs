use ears::{Sound, AudioController};
use std::collections::HashMap;
use find_folder;


pub struct SoundHandler {
    pub map: HashMap<&'static str, Sound>,
}

impl SoundHandler {
    pub fn fill() -> Self {
        let mut map: HashMap<&str, Sound> = HashMap::new();
        for path in SOUNDS {
            // Sprites are located here Create Path
            let sound_path = match find_folder::Search::ParentsThenKids(2, 2)
                .for_folder("Sounds") {
                Ok(res) => res.join(path),
                Err(_) => panic!("Folder not found!"),
            };
            // Create String from path
            let sound_string = match sound_path.to_str() {
                Some(res) => res,
                None => panic!("Sound not found"),
            };

            // Open path an create image
            //println!("{}", sound_string);
            let sound = match Sound::new(sound_string) {
                Some(x) => x,
                None => panic!("Can't open {} in {}", path, sound_string),
            };
            map.insert(*path, sound);

        }
        SoundHandler { map: map }
    }
    pub fn play(&mut self, sound: &str) {
        let plays = match self.map.get_mut(sound) {
            Some(res) => res,
            None => {
                panic!("Can't play {}", sound);
            }
        };
        plays.play();

    }
}

const SOUNDS: &'static [&'static str] = &[
                                            "Chicken.ogg", 
                                            "Dagger.ogg",
                                            "Dead.ogg",
                                            "Item.ogg",
                                            "Spear.ogg",
                                            "Sword.ogg",
                                            "test.ogg",
                                            "Welcome.ogg",
                                            ];
