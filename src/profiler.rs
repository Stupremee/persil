//! The actual implementation of the `persil` logic.
//!
//! The module is built around the [`Profiler`] struct which
//! is meant to passed around multiple "trace points" which then
//! can be used to record events.
//!
//! [`Profiler`]: ./struct.Profiler.html

use measureme::TimingGuard;
use std::{
    error::Error,
    path::PathBuf,
    sync::atomic::{AtomicBool, Ordering},
    thread::ThreadId,
};

/// `MmapSerializationSink` is faster on macOS and Linux
/// but `FileSerializationSink` is faster on Windows
#[cfg(not(windows))]
type Sink = measureme::MmapSerializationSink;
#[cfg(windows)]
type Sink = measureme::FileSerializationSink;

/// When a `Guard` is dropped, it will stop recording the
/// event of the inner profiler.
///
/// If profiling is disabled, the `Guard` struct will do nothing.
pub struct Guard<'guard> {
    _inner: Option<TimingGuard<'guard, Sink>>,
}

/// The `Profiler` struct is used to trace events.
///
/// The events will be stored on disk, if the profiler instance
/// is dropped.
pub struct Profiler {
    profiler: measureme::Profiler<Sink>,
    enabled: AtomicBool,
}

impl Profiler {
    /// Creates a new `Profiler` with the given path.
    ///
    /// The profiling results will be stored at `<path>.events`, `<path>.strings`, etc.
    pub fn from_path(path: impl Into<PathBuf>) -> Result<Self, Box<dyn Error>> {
        let path = path.into();

        match path.parent() {
            Some(parent) if !path.exists() => {
                std::fs::create_dir_all(parent)?;
            }
            _ => {}
        }

        Ok(Self {
            profiler: measureme::Profiler::new(path.as_ref())?,
            enabled: AtomicBool::default(),
        })
    }

    /// Creates a new `Profiler` from a given application name.
    ///
    /// The profiling results will be stored at `./trace/<name>-<pid>.events`, etc.
    pub fn from_name(name: impl AsRef<str>) -> Result<Self, Box<dyn Error>> {
        let path = format!("./trace/{}-{}", name.as_ref(), std::process::id());
        Self::from_path(path)
    }

    /// Enables the `Profiler`.
    ///
    /// A `Profiler`, that is created via [`from_path`] or [`from_name`],
    /// is disabled by default so you have to enable it first using this method.
    ///
    /// [`from_path`]: ./fn.from_path.html
    /// [`from_name`]: ./fn.from_name.html
    pub fn enable(&self) {
        self.enabled.store(true, Ordering::SeqCst);
    }

    /// Disables the `Profiler`.
    ///
    /// This method will make the profiler stop recording any new events
    /// but will still write all stored results to disk if dropped.
    pub fn disable(&self) {
        self.enabled.store(false, Ordering::SeqCst);
    }

    /// Starts profiling an event with the given `category` and `label`.
    pub fn trace(&self, category: &str, label: &str) -> Guard<'_> {
        if !self.enabled.load(Ordering::SeqCst) {
            return Guard { _inner: None };
        }

        let kind = self.profiler.alloc_string(category);

        let label = self.profiler.alloc_string(label);
        let id = measureme::EventId::from_label(label);
        let thread_id = current_thread_id() as u32;

        let inner = self
            .profiler
            .start_recording_interval_event(kind, id, thread_id);
        Guard {
            _inner: Some(inner),
        }
    }
}

/// Gets the current thread id and transmutes it into a
/// `u64`.
fn current_thread_id() -> u64 {
    // TODO: Remove unsafe if https://github.com/rust-lang/rust/issues/67939 is resolved.
    let tid = std::thread::current().id();
    unsafe { std::mem::transmute::<ThreadId, u64>(tid) }
}
