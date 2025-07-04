#[cfg(test)]
mod tests {
    use prepatory::ring::RingBuffer;

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

