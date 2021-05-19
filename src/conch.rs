use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::RwLock;

#[derive(Debug)]
pub struct Conch<K: Hash + Clone + Eq, V: Clone> {
    maps: Vec<RwLock<HashMap<K, V>>>,
}

impl<K: Hash + Clone + Eq, V: Clone> Conch<K, V> {
    pub fn new(bucket_size: usize) -> Conch<K, V> {
        assert!(bucket_size >= 1);
        let mut vec = Vec::with_capacity(bucket_size);
        for _ in 1..=bucket_size {
            vec.push(RwLock::new(HashMap::with_capacity(32)))
        }
        Conch { maps: vec }
    }

    pub fn update(&mut self, key: K, v: V) {
        let index = self.get_index(key.clone());
        let map = &self.maps[index];
        let mut map = map.write().unwrap();
        map.insert(key, v);
    }

    pub fn get(&mut self, key: K) -> Option<V> {
        let index = self.get_index(key.clone());
        let map = &self.maps[index];
        let map = map.read().unwrap();
        map.get(&key).cloned()
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.get_index(key.clone());
        let map = &self.maps[index];
        let mut map = map.write().unwrap();
        map.remove(key)
    }

    fn get_index(&self, key: K) -> usize {
        let hash = get_hash(key);
        hash as usize % self.maps.len()
    }
}

fn get_hash<K: Hash>(obj: K) -> u64 {
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_work_even_with_one_bucket() {
        let mut conch = Conch::new(1);
        conch.update("foo", "bar");
        conch.update("bing", "baz");
        conch.update("barf", "bong");
        assert_eq!(conch.get("foo"), Some("bar"));
        assert_eq!(conch.get("bing"), Some("baz"));
        assert_eq!(conch.get("barf"), Some("bong"));
        conch.remove(&"barf");
        assert_eq!(conch.get("barf"), None);
    }
}
