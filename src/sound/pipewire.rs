use super::Mute;
use crate::sh;

#[derive(Default)]
pub struct PipeWireServer();

const DEFAULT_SINK: &str = "@DEFAULT_AUDIO_SINK@";

const MUTE: Mute = Mute::Mute("1");
const UNMUTE: Mute = Mute::Unmute("0");
const TOGGLE: Mute = Mute::Toggle("toggle");

impl super::Server for PipeWireServer {
    fn default_sink(&self) -> Option<String> {
        Some(String::from(DEFAULT_SINK))
    }

    fn sink_exists(&self, sink: Option<&str>) -> bool {
        sh::exec("wpctl", &["inspect", sink.unwrap_or(DEFAULT_SINK)]).is_ok()
    }

    fn sink_is_muted(&self, sink: &str) -> bool {
        let output = sh::exec_with_output("wpctl", &["get-volume", sink])
            .expect("failed to get volume info");

        let s = String::from_utf8(output).expect("output is not utf-8");
        s.contains("[MUTED]")
    }

    fn mute_sink(&self, sink: &str) -> Result<(), ()> {
        set_mute(sink, MUTE)
    }

    fn unmute_sink(&self, sink: &str) -> Result<(), ()> {
        set_mute(sink, UNMUTE)
    }

    fn toggle_sink(&self, sink: &str) -> Result<(), ()> {
        set_mute(sink, TOGGLE)
    }
}

fn set_mute(sink: &str, mute: super::Mute) -> Result<(), ()> {
    sh::exec("wpctl", &["set-mute", sink, mute.to_str()]).map_err(|_| ())
}

#[cfg(test)]
#[cfg(target_os = "linux")]
mod tests {
    use super::PipeWireServer;
    use crate::sound::Server;

    #[test]
    fn test_pipewire() {
        let pipewire = PipeWireServer::default();
        println!("{}", pipewire.sink_exists(None));
    }
}
