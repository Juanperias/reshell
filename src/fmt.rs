#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {  
        $crate::dos::put::put_args(format_args!($($arg)*));        
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n");
    };
    ($($arg:tt)*) => {
        $crate::print!($($arg)*);
        $crate::print!("\n");
    }
}
