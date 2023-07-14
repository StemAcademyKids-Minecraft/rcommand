
#[macro_export]
macro_rules! command {
    ($e:expr) => { $crate::mcommand($e.to_string())};
    ($fmt:expr, $($arg:tt)*) => { $crate::mcommand(format!($fmt, $($arg)*))};
}
#[macro_export]
macro_rules! delay {
    ($e:expr) => {
        $crate::mdelay($e)
    };
}


#[macro_export]
macro_rules! playsound {
    ($e:expr) => { $crate::mplaysound($e)};
    ($fmt:expr, $($arg:tt)*) => { $crate::mplaysound(format!($fmt, $($arg)*))};
}