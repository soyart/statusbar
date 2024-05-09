use self::pulse::PulseServer;

mod alsa;
mod pipewire;
mod pulse;

pub(crate) trait Server {
    fn init(&mut self) {}

    fn default_sink(&self) -> Option<String>;
    fn sink_exists(&self, sink: Option<&str>) -> bool;
    fn sink_is_muted(&self, sink: &str) -> bool;
}

pub(crate) enum Sound {
    ALSA,
    PulseAudio(PulseServer),
    Pipewire,
}

impl std::fmt::Debug for Sound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ALSA => write!(f, "ALSA"),
            Self::PulseAudio(_) => write!(f, "PulseAudio"),
            Self::Pipewire => write!(f, "PipeWire"),
        }
    }
}

impl Sound {
    fn auto_detect() -> Option<Self> {
        todo!()
    }

    fn default_sink(&self) -> Option<String> {
        todo!()
    }

    fn volume(&self, sink: Option<&str>) -> f32 {
        todo!()
    }

    fn vol_incr_percent(&self, sink: Option<&str>, percent: u8) {}

    fn vol_decr_percent(&self, sink: Option<&str>, percent: u8) {}
}
