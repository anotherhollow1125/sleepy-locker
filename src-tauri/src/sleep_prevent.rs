use crate::{LockState, Mode};
use windows::Win32::System::Power::{
    SetThreadExecutionState, ES_CONTINUOUS, ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED,
};

pub fn prevent_sleep() {
    unsafe {
        SetThreadExecutionState(ES_SYSTEM_REQUIRED | ES_DISPLAY_REQUIRED | ES_CONTINUOUS);
    }
}

pub fn allow_sleep() {
    unsafe {
        SetThreadExecutionState(ES_CONTINUOUS);
    }
}

pub(crate) fn execute_prevent_or_allow(lock_state: &LockState) {
    match lock_state {
        LockState::Unlock(Mode::Prevent) => prevent_sleep(),
        _ => allow_sleep(),
    }
}
