pub mod notifications {
    use notify_rust::{Notification, Timeout};

    pub fn send_notif(content: &str) {
        Notification::new()
            .auto_icon()
            .appname("Clarity")
            .body(content)
            .timeout(Timeout::Milliseconds(6000))
            .show()
            .unwrap();
    }
}
