
//need to change for mod.rs for module identification
// === 1. Rust CLI Watchdog for Desktop ===
// File: src/main.rs
use std::process::{Command, Child};
use std::time::Duration;
use tokio::{time, process::Command as TokioCommand};
use tracing::{info, error};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    loop {
        let mut child: Option<Child> = None;  //Child:Representation of a running or exited child process.
        // let path = r#"C:\Users\user\Downloads\__MACOSX\Dobby for windows\dobby_for_windows\src-tauri\target
        // \release\dobby-for-windows.exe"#;
        let path = std::path::Path::new("bin").join("dobby-for-windows.exe");
        info!("Starting Dobby...");
        match Command::new(path).spawn() {  //executes as child process returns a handle to it
            Ok(process) => {
                info!("App started with PID: {}", process.id());
                child = Some(process);
            },
            Err(e) => error!("Failed to start app: {}", e),
        }

        loop {
            time::sleep(Duration::from_secs(5)).await;  //if this delay is not there then system will slow down
            //constantly calling try_wait() millions of times per second.  slows down machine

            if let Some(ref mut process) = child {
                match process.try_wait() {  // Non-blocking
                    // Attempts to collect the exit status of the child if it has already exited.
                    Ok(Some(status)) => {
                        error!("App exited with status: {}", status);
                        break;
                    }
                    Ok(None) => continue,
                    Err(e) => {
                        error!("Failed to check process: {}", e);
                        break;
                    }
                }
            } else {
                break;
            }
        }

        info!("Restarting app...");
    }
}

// Add to Cargo.toml:
// [dependencies]
// tokio = { version = "1", features = ["full"] }
// tracing = "0.1"
// tracing-subscriber = "0.3"


// === 2. Mobile-Friendly Shared Rust Lib Template ===
// File: src/lib.rs
// #[no_mangle]
// pub extern "C" fn start_watchdog() {
//     // Placeholder: Add monitoring logic here
//     println!("Watchdog started (mobile)");
// }

// Android Integration Example (Kotlin):
/*
external fun start_watchdog()

init {
    System.loadLibrary("your_rust_lib")
    start_watchdog()
}

// In build.gradle:
// android.defaultConfig.externalNativeBuild.ndkBuild.cppFlags += "-frtti -fexceptions"
*/

// iOS Integration Example (Swift):
/*
// Bridging Header:
// #include "your_rust_header.h"

@objc class WatchdogBridge: NSObject {
    @objc func start() {
        start_watchdog()
    }
}
*/


// === 3. Auto-Restart Setup Notes ===

/*
Linux (systemd):

[Unit]
Description=Tauri App Watchdog
After=network.target

[Service]
ExecStart=/path/to/watchdog
Restart=always
RestartSec=3

[Install]
WantedBy=multi-user.target

# Save as /etc/systemd/system/tauri-watchdog.service
# Run:
# sudo systemctl enable tauri-watchdog
# sudo systemctl start tauri-watchdog

macOS (launchd):

<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key>
  <string>com.yourcompany.watchdog</string>
  <key>ProgramArguments</key>
  <array>
    <string>/path/to/watchdog</string>
  </array>
  <key>RunAtLoad</key>
  <true/>
  <key>KeepAlive</key>
  <true/>
</dict>
</plist>
# Save as ~/Library/LaunchAgents/com.yourcompany.watchdog.plist
# Run:
# launchctl load ~/Library/LaunchAgents/com.yourcompany.watchdog.plist

Windows (Task Scheduler):
- Use `schtasks /Create` to make a repeating watchdog job
- Example:
  schtasks /Create /SC MINUTE /MO 1 /TN "TauriWatchdog" /TR "C:\path\to\watchdog.exe" /RL HIGHEST
*/





// ******************************************Notes*************************************
/*
If you don’t want to use Option, you could write it like this — but you'd lose flexibility:

let mut child = Command::new(path).spawn().expect("Failed to start");
That forces a crash if it can't start. Using Option lets you gracefully handle failure with a retry, logging, etc.

child = Some(process) is needed because child is an Option<Child>

Option is used to allow retry and error handling if the process fails to start

Without it, you'd either crash or need extra error handling logic

*/
