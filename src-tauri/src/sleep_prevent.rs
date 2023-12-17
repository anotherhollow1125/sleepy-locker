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
