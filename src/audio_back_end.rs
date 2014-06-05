
#![allow(missing_doc)]

use {
    Music,
    Sound,
};

pub trait AudioBackEnd<M: Music, S: Sound> {
    fn play_sound(&self, sound: &S);
    /*
    fn set_sound_volumn(&mut self, sound: &S, volumn: f64);
    fn get_sound_volumn(&mut self, sound: &S) -> f64;

    fn play_music(&self, music: &M);
    fn set_music_volumn(&mut self, music: &M, volumn: f64);
    fn get_music_volumn(&mut self, music: &M) -> f64;
    */
}

