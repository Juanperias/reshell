#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {  
        $crate::dos::put::put_args(format_args!($($arg)*));        
    };
}
