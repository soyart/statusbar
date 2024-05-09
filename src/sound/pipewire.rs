use crate::sh;

#[derive(Default)]
pub(super) struct PipewireServer();

const DEFAULT_SINK: &str = "@DEFAULT_AUDIO_SINK@";

impl super::Server for PipewireServer {
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
}

#[cfg(test)]
#[cfg(target_os = "linux")]
mod tests {
    use super::PipewireServer;
    use crate::sound::Server;

    #[test]
    fn test_pipewire() {
        let pipewire = PipewireServer::default();
        println!("{}", pipewire.sink_exists(None));
    }
}
