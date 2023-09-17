#[macro_export]
macro_rules! info {
    ($target:expr, $($arg:tt)+) => {
        println!("\x1b[38;5;045m{}\x1b[m", format!($target,  $($arg)+))
    };

    ($($arg:tt)+) => {
        println!("\x1b[38;5;045m{}\x1b[m", format!("{}", $($arg)+))
    };
}

#[macro_export]
macro_rules! warn {
    ($target:expr, $($arg:tt)+) => {
        println!("\x1b[38;5;226m{}\x1b[m", format!($target,  $($arg)+))
    };

    ($($arg:tt)+) => {
        println!("\x1b[38;5;226m{}\x1b[m", format!("{}", $($arg)+))
    };
}

#[macro_export]
macro_rules! error {
    ($target:expr, $($arg:tt)+) => {
        println!("\x1b[38;5;196m{}\x1b[m", format!($target,  $($arg)+))
    };

    ($($arg:tt)+) => {
        println!("\x1b[38;5;196m{}\x1b[m", format!("{}", $($arg)+))
    };
}


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        info!("Hello World");
        warn!("Hello {}", "world");
        error!("{} {} {}", "Hello", "World", "!");
    }

    #[test]
    fn main() {
        for color in 0..256 {
            print!("\x1b[38;5;{0}mColor{0:03}\x1b[m ", color);
            if color % 8 == 7 {
                println!("");
            }
        }
    }
}
