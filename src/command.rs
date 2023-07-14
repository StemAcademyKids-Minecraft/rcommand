pub static mut COMMANDS: Vec<String> = Vec::new();

#[macro_export]
macro_rules! command {
    ($e:expr) => { $crate::command($e.to_string())};
    ($fmt:expr, $($arg:tt)*) => { $crate::command(format!($fmt, $($arg)*))};
}

#[macro_export]
macro_rules! delay {
    ($e:expr) => {
        $crate::delay($e)
    };
}

#[inline]
pub fn sleep_ms(ms: u64) {
    use std::{thread, time};
    let ms = time::Duration::from_millis(ms);
    thread::sleep(ms);
}

pub fn delay(tick: u32) {
    //20tick = 1s
    let delay = "&dly".to_string() + &tick.to_string();
    
    unsafe { 
        COMMANDS.push(delay+"@:") 
    };
}

pub fn command(comm: String) {
    unsafe {
        COMMANDS.push(comm+"@:");
    }
}

pub fn get_commands() -> Vec<String> {
    unsafe {
        return COMMANDS.clone();
    }
}

pub fn clear_commands() {
    unsafe {
        return COMMANDS.clear();
    }
}