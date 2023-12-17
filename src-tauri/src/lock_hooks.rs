use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tauri::{App, Manager};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::RemoteDesktop::{
    WTSRegisterSessionNotification, NOTIFY_FOR_THIS_SESSION,
};
use windows::Win32::System::Threading::GetCurrentThreadId;
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, SetWindowsHookExW, UnhookWindowsHookEx, HHOOK, WH_GETMESSAGE,
    WM_WTSSESSION_CHANGE, WTS_SESSION_LOCK, WTS_SESSION_UNLOCK,
};

static HHK: Lazy<Mutex<Option<HHOOK>>> = Lazy::new(|| Mutex::new(None));

unsafe extern "system" fn gm_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let Some(hhk) = HHK.lock().unwrap().clone().take() else {
        return LRESULT(0);
    };

    if code == WM_WTSSESSION_CHANGE as i32 {
        dbg!("WM_WTSSESSION_CHANGE");
        match wparam.0 as u32 {
            WTS_SESSION_LOCK => {
                dbg!("WTS_SESSION_LOCK");
            }
            WTS_SESSION_UNLOCK => {
                dbg!("WTS_SESSION_UNLOCK");
            }
            _ => {}
        }
    }

    let ret = CallNextHookEx(hhk, code, wparam, lparam);
    ret
}

pub fn set_hook(app: &mut App) -> Result<()> {
    let hwnd = app.get_window("main").unwrap().hwnd().unwrap();
    let hwnd = HWND(hwnd.0);

    let hhk = unsafe {
        WTSRegisterSessionNotification(hwnd, NOTIFY_FOR_THIS_SESSION)
            .with_context(|| format!("@{}:{}", file!(), line!()))?;
        SetWindowsHookExW(WH_GETMESSAGE, Some(gm_proc), None, GetCurrentThreadId())
            .with_context(|| format!("@{}:{}", file!(), line!()))?
    };

    let mut hhk_l = HHK.lock().unwrap();
    *hhk_l = Some(hhk);

    Ok(())
}

pub fn unhook() -> Result<()> {
    let hhk = HHK.lock().unwrap().take().unwrap();

    unsafe {
        UnhookWindowsHookEx(hhk).with_context(|| format!("@{}:{}", file!(), line!()))?;
    }

    Ok(())
}
