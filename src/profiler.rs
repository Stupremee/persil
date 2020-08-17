//! The actual implementation of the `persil` logic.
//!
//! The module is built around the [`Profiler`].
//! There's one global instance that will be used by every
//! [`trace`] call and can be initialized with [`persil::init`].
//!
//! [`Profiler`]: ./struct.Profiler.html
//! [`trace`]: ../fn.trace.html
//! [`persil::init`]: ../fn.init.html

use measureme::TimingGuard;
use std::{
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
    thread::ThreadId,
};

/// `MmapSerializationSink` is faster on macOS and Linux
/// but `FileSerializationSink` is faster on Windows
#[cfg(not(windows))]
type Sink = measureme::MmapSerializationSink;
#[cfg(windows)]
type Sink = measureme::FileSerializationSink;

/// Indicates if the profiler will trace events.
const ENABLED: AtomicBool = AtomicBool::new(false);

/// Enables the global profiler.
///
/// The profiler will only emit results if it's enabled using this method.
pub fn enable() {
    ENABLED.store(true, Ordering::SeqCst)
}

/// Disables the global profiler.
///
/// After this method is called, there will be no results emitted and
/// every [`trace`] call is basically a no-op.
///
/// [`trace`]: ../fn.trace.html
pub fn disable() {
    ENABLED.store(false, Ordering::SeqCst)
}

/// When a `Guard` is dropped, it will stop recording the
/// event of the inner profiler.
///
/// If profiling is disabled, the `Guard` struct will do nothing.
pub struct Guard<'guard> {
    _inner: Option<TimingGuard<'guard, Sink>>,
}

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
    pub(crate) fn trace(&self, category: &str, label: &str) -> Guard<'_> {
        if ENABLED.load(Ordering::Relaxed) {
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
