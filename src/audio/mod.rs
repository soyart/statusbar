mod alsa;
mod pulse;
mod pipewire;

pub struct SoundServer {
	target_sink: String,
	step_size: u8,
}

pub enum Server {
	ALSA(SoundServer),
	PulseAudio(SoundServer),
	Pipewire(SoundServer),	
}

impl std::fmt::Debug for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::ALSA(SoundServer { target_sink, step_size: _ }) => write!(f, "ALSA"),
			Self::PulseAudio => write!(f, "PulseAudio"),
			Self::Pipewire => write!(f, "PipeWire"),
		}
	}
}

pub impl Server {
	fn auto_detect() -> Option<Self> {
		todo!()
	}

	fn default_sink(&self) -> Option<String> {
		todo!()
	}

	fn volume(&self, sink: Option<&str>) -> f32 {
		
	}

	fn vol_incr_percent(&self, sink: Option<&str>, percent: u8) {
		
	}

	fn vol_decr_percent(&self, sink:Option<&str>, percent: u8) {
		
	}
}
