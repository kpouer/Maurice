use std::time::Instant;

pub(crate) struct Message {
    message: String,
    time: Instant,
}

impl Message {
    pub(crate) fn new(message: String) -> Self {
        Self {
            message,
            time: Instant::now(),
        }
    }

    pub(crate) fn message(&self) -> &str {
        &self.message
    }

    pub(crate) fn is_expired(&self) -> bool {
        self.time.elapsed().as_secs() > 1
    }
}
