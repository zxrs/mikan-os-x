pub const MEMORY_MAP_SIZE: usize = 4096 * 4;

#[derive(Debug)]
pub struct MemoryMap {
    pub size: usize,
    pub buf: [u8; MEMORY_MAP_SIZE],
    pub map_key: usize,
    pub descriptor_size: usize,
    pub version: u32,
}

impl Default for MemoryMap {
    fn default() -> Self {
        MemoryMap {
            size: 0,
            buf: [0; MEMORY_MAP_SIZE],
            map_key: 0,
            descriptor_size: 0,
            version: 0,
        }
    }
}
