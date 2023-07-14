

pub static mut COMMANDS: Vec<String> = Vec::new();





pub fn mdelay(tick: u32) {
    //20tick = 1s
    let delay = "&dly".to_string() + &tick.to_string();
    
    unsafe { 
        COMMANDS.push(delay+"@:") 
    };
}

pub fn mcommand(comm: String) {
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