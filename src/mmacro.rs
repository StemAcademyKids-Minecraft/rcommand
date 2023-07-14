
use crate::commands;
use crate::sound;

use commands::mcommand;
use commands::mdelay;
use sound::mplaysound;


#[macro_export]
macro_rules! command {
    ($e:expr) => { $crate::commands::mcommand($e.to_string())};
    ($fmt:expr, $($arg:tt)*) => { $crate::commands::mcommand(format!($fmt, $($arg)*))};
}
#[macro_export]
macro_rules! delay {
    ($e:expr) => {
        $crate::commands::mdelay($e)
    };
}


#[macro_export]
macro_rules! playsound {
    ($e:expr,$t:expr) => { $crate::sound::mplaysound($e,$t)};
}