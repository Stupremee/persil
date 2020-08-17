//! The actual implementation of the `persil` logic.
//!
//! The module is built around the [`Profiler`].
//! There's one global instance that will be used by every
//! [`trace`] call and can be initialized with [`persil::init`].

pub(crate) use measureme::TimingGuard;
use std::{path::Path, thread::ThreadId};

/// `MmapSerializationSink` is faster on macOS and Linux
/// but `FileSerializationSink` is faster on Windows
#[cfg(not(windows))]
pub(crate) type Sink = measureme::MmapSerializationSink;
#[cfg(windows)]
pub(crate) type Sink = measureme::FileSerializationSink;

/// The `Profiler` struct is used to start tracing events.
pub(crate) struct Profiler {
    profiler: measureme::Profiler<Sink>,
}

impl Profiler {
    /// Creates a new `Profiler` with the given path.
    pub(crate) fn new(path: &Path) -> Profiler {
        Self {
            profiler: measureme::Profiler::new(path).expect("failed to create profiler"),
        }
    }

    /// Starts profiling an event with the given `category` and `label`.
    pub(crate) fn trace(&self, category: &str, label: &str) -> TimingGuard<'_, Sink> {
        let kind = self.profiler.alloc_string(category);

        let label = self.profiler.alloc_string(label);
        let id = measureme::EventId::from_label(label);
        let thread_id = current_thread_id() as u32;

        self.profiler
            .start_recording_interval_event(kind, id, thread_id)
    }
}

/// Gets the current thread id and transmutes it into a
/// `u64`.
fn current_thread_id() -> u64 {
    // TODO: Remove unsafe if https://github.com/rust-lang/rust/issues/67939 is resolved.
    let tid = std::thread::current().id();
    unsafe { std::mem::transmute::<ThreadId, u64>(tid) }
}
