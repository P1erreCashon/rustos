use crate::sbi;
use core::fmt::{self,Write};

struct Stdio;

impl Write for Stdio{
    fn write_str(&mut self,s:&str) ->fmt::Result{
        for c in s.chars(){
            sbi::console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args:fmt::Arguments){
    Stdio.write_fmt(args).unwrap();
}


/// print string macro
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

/// println string macro
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
