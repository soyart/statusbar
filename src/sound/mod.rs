mod alsa;
mod pipewire;
mod pulseaudio;

use self::{pipewire::PipeWireServer, pulseaudio::PulseServer};

pub(crate) trait Server {
    fn init(&mut self) {}

    fn default_sink(&self) -> Option<String>;
    fn sink_exists(&self, sink: Option<&str>) -> bool;
    fn sink_is_muted(&self, sink: &str) -> bool;
    fn mute_sink(&self, sink: &str) -> Result<(), ()>;
    fn unmute_sink(&self, sink: &str) -> Result<(), ()>;
    fn toggle_sink(&self, sink: &str) -> Result<(), ()>;
}

pub(crate) enum Sound {
    ALSA,
    PulseAudio(PulseServer),
    Pipewire(PipeWireServer),
}

pub enum Mute {
    Mute(&'static str),
    Unmute(&'static str),
    Toggle(&'static str),
}

impl Mute {
    fn to_str(&self) -> &str {
        match self {
            Self::Mute(v) => v,
            Self::Unmute(v) => v,
            Self::Toggle(v) => v,
        }
    }

    fn to_string(&self) -> String {
        self.to_str().to_string()
    }
}

impl Sound {
    fn auto_detect() -> Option<Self> {
        todo!()
    }

    fn default_sink(&self) -> Option<String> {
        match self {
            Self::PulseAudio(server) => server.default_sink(),
            Self::Pipewire(server) => server.default_sink(),
            _ => {
                panic!("{} not implemented", self);
            }
        }
    }

    fn volume(&self, sink: Option<&str>) -> f32 {
        todo!()
    }

    fn vol_incr_percent(&self, sink: Option<&str>, percent: u8) {}

    fn vol_decr_percent(&self, sink: Option<&str>, percent: u8) {}
}

impl std::fmt::Debug for Sound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for Sound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ALSA => write!(f, "ALSA"),
            Self::PulseAudio(_) => write!(f, "PulseAudio"),
            Self::Pipewire(_) => write!(f, "PipeWire"),
        }
    }
}

pub fn floor(percent: u8) -> u8 {
    if percent > 100 {
        return 100;
    }

    if percent < 0 {
        return 0;
    }

    percent
}
