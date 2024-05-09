use crate::sh;

#[derive(Default)]
pub struct PulseServer(pub String);

impl super::Server for PulseServer {
    fn init(&mut self) {
        let default_sink = self.default_sink();
        self.0 = default_sink.unwrap();
    }

    fn default_sink(&self) -> Option<String> {
        if !self.0.is_empty() {
            return Some(self.0.clone());
        }

        let out = sh::exec_with_output("pactl", &["get-default-sink"])
            .expect("failed to get pulseaudio default sink");

        Some(String::from_utf8(out).expect("pulseaudio sink name is not utf-8"))
    }

    fn sink_exists(&self, sink: Option<&str>) -> bool {
        sh::exec(
            "pactl",
            &["get-sink-volume", sink.unwrap_or(self.0.as_str())],
        )
        .is_ok()
    }

    fn sink_is_muted(&self, sink: &str) -> bool {
        sh::exec("pactl", &["get-sink-muted", sink]).is_ok_and(|output| false)
    }
}
