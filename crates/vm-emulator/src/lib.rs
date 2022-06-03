pub struct PageTableEntry (u32);

impl PageTableEntry {
    fn size() -> usize {
        std::mem::size_of::<u32>()
    }

    /// Returns the virtual page number (VPN) for the address.
    fn page_number(&self) -> u32 {
        todo!()
    }

    /// Returns true if the page is allocated and the requester has sufficient
    /// permission to access the page.
    fn valid(&self) -> bool {
        todo!()
    }
}

pub struct PageTable {
    size: usize,
    page_size: usize,
    buffer: Vec<u8>,
}

impl PageTable {
    fn new(size: usize, page_size: usize) -> Self {
        todo!()
    }

    /// Returns the size of each page table entry in number of bytes.
    pub fn entry_size(&self) -> usize {
        PageTableEntry::size()
    }

    /// The minimum virtual address size possible.
    ///
    /// Space in the address is required for the page directory number, the PD
    /// offset, and the offset into the memory page being referenced.
    pub fn min_address_size(&self) -> usize {
        todo!()
    }
}

pub struct AddressSpace {
    size: usize,
    page_size: usize,
    buffer: Vec<u8>,
}

impl AddressSpace {
    fn new(size: usize, page_size: usize) -> Self {
        Self {
            size, page_size,
            buffer: Vec::with_capacity(size),
        }
    }

    /// Returns the maximum number of pages available in the address space.
    pub fn page_count(&self) -> usize {
        self.size / self.page_size
    }

    /// Returns the minimum address size required in number of bits.
    ///
    /// log2(page_size) + log2(page_count) + log2(page_size / entry_size)
    pub fn min_address_size(&self) -> usize {
        todo!()
    }

    /// Returns the address in the address space given a page number of offset.
    pub fn page<'a>(&self, n: usize, offset: usize) -> usize {
        let vaddr = n * self.page_size + offset;

        if vaddr > (self.size - 1) {
            panic!("seg fault")
        }

        vaddr
    }
}

pub struct AddressSpaceBuilder {
    size: Option<usize>,
    page_size: Option<usize>,
}

impl AddressSpaceBuilder {
    fn new() -> Self {
        Self {
            size: None,
            page_size: None,
        }
    }

    fn with_page_size(mut self, size: usize) -> Self {
        self.page_size = Some(size);

        self
    }

    fn with_size(mut self, size: usize) -> Self {
        self.size = Some(size);

        self
    }

    fn build(&mut self) -> AddressSpace {
        AddressSpace::new(
            self.size.unwrap_or(16 * 1024),
            self.page_size.unwrap_or(512))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build_address_space() {
        let address_space = AddressSpaceBuilder::new()
            .with_size(16 * 1024)
            .with_page_size(512)
            .build();

        assert_eq!(32, address_space.page_count())
    }

    #[test]
    fn can_get_address() {
        let address_space = AddressSpaceBuilder::new()
            .with_size(16 * 1024)
            .with_page_size(512)
            .build();

        assert_eq!(0, address_space.page(0, 0));
        assert_eq!(16383, address_space.page(31, 511));
    }

    #[test]
    #[should_panic]
    fn reference_out_of_address_space_panics() {
        let address_space = AddressSpaceBuilder::new()
            .with_size(16 * 1024)
            .with_page_size(512)
            .build();

        assert_eq!(16383, address_space.page(31, 512));
    }
}
