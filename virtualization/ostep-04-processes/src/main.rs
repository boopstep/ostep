use std::{env, thread, time};

fn main() {
    let a: Vec<String> = env::args().collect();

     loop {
        println!("{:?}", a[1]);
        thread::sleep(time::Duration::from_millis(1200));
    }
}
