use std::process;

extern {
    fn fork() -> u32;
    fn getpid() -> u32;
}

fn main() {
    unsafe {
        let rc: u32 = fork();

        if rc == 0 {
            println!("child! {:?}", getpid());
        } else {
            println!("parent {:?}", getpid());
        }
    }

    println!("my pid is {}", process::id())
}
