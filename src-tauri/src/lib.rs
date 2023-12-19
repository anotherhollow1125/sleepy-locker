use crate::lock_hooks::detect_lock_init;
use crate::sleep_prevent::execute_prevent_or_allow;
use std::sync::{
    mpsc::{channel, Sender},
    Arc, Mutex,
};
use std::thread::JoinHandle;

pub mod lock_hooks;
pub mod sleep_prevent;

pub enum Event {
    Lock,
    Unlock,
    Prevent,
    Allow,
    Quit,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Mode {
    Prevent,
    Allow,
}

#[derive(Clone, Copy)]
pub enum LockState {
    Lock(Mode),
    Unlock(Mode),
}

impl LockState {
    fn set_enabled(&mut self, enabled: bool) {
        let new_mode = if enabled { Mode::Prevent } else { Mode::Allow };
        match self {
            LockState::Lock(mode) => *mode = new_mode,
            LockState::Unlock(mode) => *mode = new_mode,
        }
    }

    pub fn is_enabled(&self) -> bool {
        match self {
            LockState::Lock(mode) => *mode == Mode::Prevent,
            LockState::Unlock(mode) => *mode == Mode::Prevent,
        }
    }

    fn lock(&mut self) {
        match self {
            LockState::Lock(_) => {}
            LockState::Unlock(mode) => *self = LockState::Lock(*mode),
        }
    }

    fn unlock(&mut self) {
        match self {
            LockState::Lock(mode) => *self = LockState::Unlock(*mode),
            LockState::Unlock(_) => {}
        }
    }
}

pub fn init_event_thread() -> (
    Sender<Event>,
    Arc<Mutex<LockState>>,
    impl FnOnce() -> (),
    JoinHandle<()>,
    JoinHandle<()>,
) {
    let (tx, rx) = channel();
    let tx1 = tx.clone();
    let (dw_handle, close_dummy_window) = detect_lock_init(tx1).unwrap();
    let state = Arc::new(Mutex::new(LockState::Unlock(Mode::Allow)));
    let st = state.clone();

    let event_handle = std::thread::spawn(move || {
        for event in rx {
            let mut state = st.lock().unwrap();
            match event {
                Event::Lock => {
                    state.lock();
                }
                Event::Unlock => {
                    state.unlock();
                }
                Event::Prevent => {
                    state.set_enabled(true);
                }
                Event::Allow => {
                    state.set_enabled(false);
                }
                Event::Quit => {
                    break;
                }
            }
            execute_prevent_or_allow(&state);
        }
    });

    (tx, state, close_dummy_window, dw_handle, event_handle)
}
