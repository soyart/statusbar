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
