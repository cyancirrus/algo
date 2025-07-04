mod tests {
    use prepatory::ring::RingBuffer;
    use std::sync::atomic::{AtomicUsize, Ordering};
    static LIVE_COUNT: AtomicUsize = AtomicUsize::new(0);
    struct Tracker(u32);
    impl Tracker {
        fn new(val: u32) -> Self {
            LIVE_COUNT.fetch_add(1, Ordering::SeqCst);
            Tracker(val)
        }
        fn get(&self) -> u32 {
            self.0
        }
    }

    impl Drop for Tracker {
        fn drop(&mut self) {
            LIVE_COUNT.fetch_sub(1, Ordering::SeqCst);
        }
    }
    #[test]
    fn test_ringbuffer_memory_cleanup() {
        {
            let mut rb = RingBuffer::new(3);
            rb.push_back(Tracker::new(1));
            rb.push_back(Tracker::new(2));
            rb.push_back(Tracker::new(3));
            rb.push_back(Tracker::new(4)); // overwrite 1
            assert_eq!(LIVE_COUNT.load(Ordering::SeqCst), 3);
        }
        // At this point, RingBuffer should be dropped and free all elements
        assert_eq!(LIVE_COUNT.load(Ordering::SeqCst), 0);
        let t = Tracker::new(42);
        assert_eq!(t.get(), 42);
    }

    #[test]
    fn test_mutation_memory_cleanup() {
        let mut rb = RingBuffer::new(3);
        rb.pop_front();
        assert_eq!(LIVE_COUNT.load(Ordering::SeqCst), 0);
        rb.push_back(Tracker::new(1));
        assert_eq!(LIVE_COUNT.load(Ordering::SeqCst), 1);
        rb.pop_back();
        assert_eq!(LIVE_COUNT.load(Ordering::SeqCst), 0);
        rb.push_front(Tracker::new(1));
        assert_eq!(LIVE_COUNT.load(Ordering::SeqCst), 1);
        rb.push_front(Tracker::new(2));
        assert_eq!(LIVE_COUNT.load(Ordering::SeqCst), 2);
        rb.push_front(Tracker::new(3));
        assert_eq!(LIVE_COUNT.load(Ordering::SeqCst), 3);
        rb.push_front(Tracker::new(100));
        assert_eq!(LIVE_COUNT.load(Ordering::SeqCst), 3);
        rb.push_front(Tracker::new(100));
        assert_eq!(LIVE_COUNT.load(Ordering::SeqCst), 3);
        rb.push_front(Tracker::new(100));
        assert_eq!(LIVE_COUNT.load(Ordering::SeqCst), 3);
        rb.pop_back();
        assert_eq!(LIVE_COUNT.load(Ordering::SeqCst), 2);
        rb.pop_front();
        assert_eq!(LIVE_COUNT.load(Ordering::SeqCst), 1);
        rb.pop_back();
        assert_eq!(LIVE_COUNT.load(Ordering::SeqCst), 0);

    }

    #[test]
    fn test_basic_push_pop() {
        let mut rb = RingBuffer::new(3);
        rb.push_back(1);
        rb.push_back(2);
        rb.push_back(3);
        assert_eq!(rb.pop_front(), Some(1));
        rb.push_back(4);
        assert_eq!(rb.pop_front(), Some(2));
        assert_eq!(rb.pop_front(), Some(3));
        assert_eq!(rb.pop_front(), Some(4));
        assert_eq!(rb.pop_front(), None);
    }
}

