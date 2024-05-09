use std::thread::sleep;
use std::time::Duration;

use rand::Rng;
use windows::Win32::UI::WindowsAndMessaging::{
    GetSystemMetrics, SetCursorPos, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN,
};

fn main() {
    loop {
        move_cursor_random();

        sleep(Duration::from_secs(1));
    }
}

fn move_cursor_random() {
    let screen_height = unsafe { GetSystemMetrics(SM_CXVIRTUALSCREEN) };
    let screen_width = unsafe { GetSystemMetrics(SM_CYVIRTUALSCREEN) };

    let p_height = rand::thread_rng().gen_range(1..screen_height);
    let p_width = rand::thread_rng().gen_range(1..screen_width);

    unsafe { SetCursorPos(p_height, p_width).unwrap() };
}
