use ostep_13_address_spaces::HeapAllocator;

fn main() {
    let arg: String = std::env::args().skip(1).take(1).collect();
    println!("Allocating memory block of size {}.", arg);
    let size = arg.parse::<usize>().unwrap();
    let alloc = HeapAllocator::with_capacity(size);

    alloc.join.join().unwrap()
}
