#[cfg(test)]
mod tests {
    use prepatory::lru::*;

    #[test]
    fn test_basic_lru_cache_behavior() {
        let mut cache = LruCache::new(3);
        cache.update(1, "a");
        cache.update(2, "b");
        cache.update(3, "c");

        assert_eq!(cache.get(1), Some(&"a"));
        assert_eq!(cache.get(2), Some(&"b"));
        assert_eq!(cache.get(3), Some(&"c"));

        cache.update(2, "bb");
        assert_eq!(cache.get(2), Some(&"bb"));

        cache.remove(2);
        assert_eq!(cache.get(2), None);

        cache.update(4, "d");
        cache.update(5, "e");

        assert_eq!(cache.get(1), None);        // should be evicted
        assert_eq!(cache.get(3), Some(&"c"));  // still there
        assert_eq!(cache.get(5), Some(&"e"));  // new one
    }
}
