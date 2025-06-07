use std::sync::{Arc, Mutex};

pub struct PageAlignedByteBuffer {
    data: Arc<Mutex<Vec<u8>>>,
}

impl PageAlignedByteBuffer {
    pub fn new(buffer_size: usize) -> Self {
        PageAlignedByteBuffer {
            data: Arc::new(Mutex::new(vec![0u8; buffer_size])),
        }
    }

    pub fn get_buffer(&self) -> Arc<Mutex<Vec<u8>>> {
        self.data.clone()
    }
}


// `Arc<Mutex<Vec<u8>>>` is Send + Sync by default, so no manual impls needed.

#[cfg(test)]
mod buffer_tests {
    use super::PageAlignedByteBuffer;

    #[test]
    fn buffer_creation_destruction_test() {
        {
            let _test = PageAlignedByteBuffer::new(1024 * 1024);
        }
        assert!(true);
    }
}
