use std::thread::JoinHandle;

pub struct HeapAllocator {
    pub join: JoinHandle<()>,
}

impl HeapAllocator {
    pub fn with_capacity(mebibytes: usize) -> HeapAllocator {
        let total_bytes = mebibytes * (1024 * 1024);
        let mut alloc = vec![0 as u8; total_bytes];

        let join = std::thread::spawn(move || {
            let mut count = 0;
            loop {
                alloc[count] = 1;
                count += 1;
                if count == total_bytes {
                    count = 0;
                }
            }
        });

        Self { join }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocates_mebibytes_to_heap() {
        HeapAllocator::with_capacity(10);
    }
}
