use qmk_via_api::api;
#[cfg(target_os = "macos")]
use std::panic::{self, AssertUnwindSafe};
#[cfg(target_os = "macos")]
use std::sync::mpsc;
#[cfg(target_os = "macos")]
use std::thread;

/// Initialize `KeyboardApi` in a worker thread on macOS to avoid main-thread stalls
/// caused by potential IOHIDDeviceSetReport blocking behavior. On other platforms
/// this simply calls the constructor directly.
pub fn init_keyboard_api(vid: u16, pid: u16) -> Result<api::KeyboardApi, String> {
    #[cfg(target_os = "macos")]
    {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let result = panic::catch_unwind(AssertUnwindSafe(|| {
                api::KeyboardApi::new(vid, pid, 0xff60)
                    .map_err(|e| format!("Failed to connect to device ({vid:04x}:{pid:04x}): {e}"))
            }));
            let _ = tx.send(match result {
                Ok(inner) => inner,
                Err(_) => Err("Panic during HID device enumeration".to_string()),
            });
        });
        rx.recv()
            .map_err(|e| format!("Worker thread failed: {e}"))?
    }
    #[cfg(not(target_os = "macos"))]
    {
        api::KeyboardApi::new(vid, pid, 0xff60)
            .map_err(|e| format!("Failed to connect to device ({vid:04x}:{pid:04x}): {e}"))
    }
}
