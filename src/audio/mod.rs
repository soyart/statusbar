mod alsa;
mod pipewire;
mod pulse;

pub trait SoundServer {
    fn default_sink(&self) -> Option<String>;
    fn sink_exists(&self, sink: &str) -> bool;
    fn get_vol(&self, sink: &str) -> f32;
}

pub struct Ctrl {
    target_sink: String,
    step_size: u8,
}

pub enum Server {
    ALSA(Ctrl),
    PulseAudio(Ctrl),
    Pipewire(Ctrl),
}

impl std::fmt::Debug for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ALSA(Ctrl {
                target_sink,
                step_size: _,
            }) => write!(f, "ALSA"),
            Self::PulseAudio(_) => write!(f, "PulseAudio"),
            Self::Pipewire(_) => write!(f, "PipeWire"),
        }
    }
}

impl Server {
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
