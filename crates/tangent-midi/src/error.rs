use serde::Serialize;
use std::fmt;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ErrorCode {
    InvalidFile,
    UnsupportedFormat,
    UnsupportedTiming,
    LimitExceeded,
    InvalidTempo,
    ConflictingTempo,
    InvalidTrack,
    TimeOverflow,
}
#[derive(Debug, Clone, Serialize)]
pub struct ImportError {
    pub code: ErrorCode,
    pub message: String,
    pub track: Option<usize>,
    pub source_event: Option<usize>,
}
impl ImportError {
    pub(crate) fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            track: None,
            source_event: None,
        }
    }
    pub(crate) fn at(mut self, track: usize, event: usize) -> Self {
        self.track = Some(track);
        self.source_event = Some(event);
        self
    }
}
impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(t) = self.track {
            write!(
                f,
                " (track {}, event {})",
                t,
                self.source_event.unwrap_or(0)
            )?;
        }
        Ok(())
    }
}
impl std::error::Error for ImportError {}
