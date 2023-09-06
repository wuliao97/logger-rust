#[macro_export]
macro_rules! info {
    ($target:expr) => (println!("\x1b[38;5;004m{}\x1b[m", $target))
}

#[macro_export]
macro_rules! warn {
    ($target:expr) => (println!("\x1b[38;5;220m{}\x1b[m", $target))
}

#[macro_export]
macro_rules! error {
    ($target:expr) => (println!("\x1b[38;5;124m{}\x1b[m", $target))
}



#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        info!("Hello World");
        warn!("Hello World");
        error!("Hello World");
    }

    // #[test]
    // fn main() {
    //     for color in 0..256 {
    //         print!("\x1b[38;5;{0}mColor{0:03}\x1b[m ", color);
    //         if color % 8 == 7 {
    //             println!("");
    //         }
    //     }
    // }
}
