use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub fn spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner} {msg}")
            .expect("Invalid progress bar template"),
    );
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message(message.to_string());
    pb
}
