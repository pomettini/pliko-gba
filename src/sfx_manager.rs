use agb::{
    include_wav,
    sound::mixer::{ChannelId, Mixer, SoundChannel, SoundData},
};

static TITLE_MUSIC: SoundData = include_wav!("sfx/title_loop.wav");
static GAME_MUSIC: SoundData = include_wav!("sfx/game_loop.wav");

pub struct Sfx<'a> {
    mixer: Mixer<'a>,
    channel: Option<ChannelId>,
}
impl<'a> Sfx<'a> {
    pub fn frame(&mut self) {
        for _ in 0..200 {
            self.mixer.frame();
        }
    }

    pub fn play_title_theme(&mut self) {
        let mut title_music = SoundChannel::new(TITLE_MUSIC);
        title_music.should_loop();

        self.channel = self.mixer.play_sound(title_music);
    }

    pub fn play_game_theme(&mut self) {
        let mut game_music = SoundChannel::new(GAME_MUSIC);
        game_music.should_loop();

        self.channel = self.mixer.play_sound(game_music);
    }

    pub fn create(mixer: Mixer<'a>) -> Self {
        Self {
            mixer,
            channel: None,
        }
    }

    pub fn stop(&mut self) {
        self.mixer
            .channel(self.channel.as_ref().unwrap())
            .unwrap()
            .stop();
        self.frame();
    }
}
