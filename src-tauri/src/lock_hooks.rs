use crate::Event;
use anyhow::{bail, Context, Result};
use std::sync::{
    mpsc::{channel, Sender},
    OnceLock,
};
use std::thread::JoinHandle;
use windows::core::s;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::RemoteDesktop::{
    WTSRegisterSessionNotification, WTSUnRegisterSessionNotification, NOTIFY_FOR_THIS_SESSION,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExA, DefWindowProcA, DestroyWindow, DispatchMessageA, GetMessageA, PostQuitMessage,
    RegisterClassA, SendMessageA, MSG, WINDOW_EX_STYLE, WM_CLOSE, WM_DESTROY, WM_WTSSESSION_CHANGE,
    WNDCLASSA, WS_OVERLAPPEDWINDOW, WTS_SESSION_LOCK, WTS_SESSION_UNLOCK,
};

static SNDR: OnceLock<Sender<Event>> = OnceLock::new();

pub(crate) fn detect_lock_init(tx: Sender<Event>) -> Result<(JoinHandle<()>, impl FnOnce() -> ())> {
    let Ok(_) = SNDR.set(tx) else {
        bail!("detect_lock_init called more than once");
    };

    let (tx, rx) = channel();
    let dw_handle = std::thread::spawn(move || {
        dummy_window_for_detect_lock(tx).unwrap();
    });

    let hwnd = rx
        .recv()
        .with_context(|| format!("@{}:{}", file!(), line!()))?;

    let close_dummy_window = move || unsafe {
        let _ = SendMessageA(hwnd, WM_CLOSE, WPARAM(0), LPARAM(0));
    };

    Ok((dw_handle, close_dummy_window))
}

fn dummy_window_for_detect_lock(hwnd_tx: Sender<HWND>) -> Result<()> {
    unsafe {
        let instance =
            GetModuleHandleA(None).with_context(|| format!("@{}:{}", file!(), line!()))?;
        debug_assert!(instance.0 != 0);

        // dw stands for dummy_window
        let window_class = s!("sleepy_locker_dw");

        let wc = WNDCLASSA {
            hInstance: instance.into(),
            lpszClassName: window_class,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);

        let hwnd = CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            window_class,
            s!("Dummy Window"),
            WS_OVERLAPPEDWINDOW,
            0,
            0,
            0,
            0,
            None,
            None,
            instance,
            None,
        );

        hwnd_tx
            .send(hwnd)
            .with_context(|| format!("@{}:{}", file!(), line!()))?;

        WTSRegisterSessionNotification(hwnd, NOTIFY_FOR_THIS_SESSION)
            .with_context(|| format!("@{}:{}", file!(), line!()))?;

        let mut message = MSG::default();

        while GetMessageA(&mut message, None, 0, 0).into() {
            DispatchMessageA(&message);
        }

        Ok(())
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_WTSSESSION_CHANGE => {
                let Some(tx) = SNDR.get() else {
                    return DefWindowProcA(window, message, wparam, lparam);
                };

                match wparam.0 as u32 {
                    WTS_SESSION_LOCK => {
                        tx.send(Event::Lock)
                            .expect("failed to send LockEvent::Lock");
                    }
                    WTS_SESSION_UNLOCK => {
                        tx.send(Event::Unlock)
                            .expect("failed to send LockEvent::Unlock");
                    }
                    _ => {}
                }

                LRESULT(0)
            }
            WM_CLOSE => {
                WTSUnRegisterSessionNotification(window).unwrap();
                DestroyWindow(window).unwrap();
                LRESULT(0)
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
