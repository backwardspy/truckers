use std::{
    ffi::{c_char, CString},
    fs::File,
    io::Write as _,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex,
    },
};

use tracing::span;

use crate::ffi::{SCS_LOG_TYPE_ERROR, SCS_LOG_TYPE_MESSAGE, SCS_LOG_TYPE_WARNING};

struct StringVisitor {
    value: String,
}

impl StringVisitor {
    fn new() -> Self {
        Self {
            value: String::new(),
        }
    }

    fn value(&self) -> &str {
        &self.value
    }
}

impl tracing::field::Visit for StringVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.value.push_str(&format!("{:?}", value));
        } else {
            self.value
                .push_str(&format!("  {}={:?}", field.name(), value));
        }
    }
}

pub struct ETS2TracingSubscriber {
    file_log: Mutex<File>,
    game_log: extern "stdcall" fn(i32, *const c_char),
    ids: AtomicUsize,
}

impl ETS2TracingSubscriber {
    pub fn new(game_log: extern "stdcall" fn(i32, *const c_char)) -> Self {
        Self {
            file_log: Mutex::new(File::create("pigeon.truckers.log").unwrap()),
            game_log,
            ids: AtomicUsize::new(1),
        }
    }
}

impl tracing::Subscriber for ETS2TracingSubscriber {
    fn enabled(&self, _metadata: &tracing::Metadata) -> bool {
        true
    }

    fn new_span(&self, _span: &span::Attributes) -> span::Id {
        let id = self.ids.fetch_add(1, Ordering::SeqCst);
        span::Id::from_u64(id as u64)
    }

    fn record(&self, _span: &span::Id, _values: &span::Record) {}
    fn record_follows_from(&self, _span: &span::Id, _follows: &span::Id) {}

    fn event(&self, event: &tracing::Event) {
        let mut msg = String::new();

        msg.push_str(&format!(
            "[{:>5}] {}:",
            event.metadata().level().as_str(),
            event.metadata().target()
        ));

        let mut visitor = StringVisitor::new();
        event.record(&mut visitor);
        msg.push_str(visitor.value());

        let msg = CString::new(msg).unwrap();
        let log_type = match *event.metadata().level() {
            tracing::Level::ERROR => SCS_LOG_TYPE_ERROR,
            tracing::Level::WARN => SCS_LOG_TYPE_WARNING,
            _ => SCS_LOG_TYPE_MESSAGE,
        };
        writeln!(self.file_log.lock().unwrap(), "{}", msg.to_string_lossy()).unwrap();
        (self.game_log)(log_type, msg.as_ptr());
    }

    fn enter(&self, _span: &span::Id) {}
    fn exit(&self, _span: &span::Id) {}
}
