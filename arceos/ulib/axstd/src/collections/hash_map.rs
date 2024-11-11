use core::hash::{BuildHasher, Hash, Hasher};
use arceos_api::modules::axhal::misc::random;
use core::hash::BuildHasherDefault;

pub struct HashMap<K, V, S=MyBuildHasher> {
    base: hashbrown::HashMap<K, V, S>,
}

impl<K, V> HashMap<K, V, MyBuildHasher> {
    pub fn new() -> HashMap<K, V, MyBuildHasher> {
        Default::default()
    }
}

impl <K,V,S> HashMap<K,V,S> where
    K:Eq+Hash,
    S:BuildHasher
{
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.base.insert(k, v)
    }
}

impl<K, V, S> Default for HashMap<K, V, S>
where
    S: Default,
{
    /// Creates an empty `HashMap<K, V, S>`, with the `Default` value for the hasher.
    #[inline]
    fn default() -> HashMap<K, V, S> {
        HashMap::with_hasher(Default::default())
    }
}

impl<K,V,S> HashMap<K,V,S>{
    pub const fn with_hasher(hash_builder: S) -> HashMap<K, V, S> {
        HashMap { base: hashbrown::HashMap::with_hasher(hash_builder) }
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter { base: self.base.iter() }
    }
}

pub struct Iter<'a, K: 'a, V: 'a> {
    base: hashbrown::hash_map::Iter<'a, K, V>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        self.base.next()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }
    #[inline]
    fn count(self) -> usize {
        self.base.len()
    }
    #[inline]
    fn fold<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.base.fold(init, f)
    }
}

pub struct MyHasher{
    hash:u64
}

impl Hasher for MyHasher {
    fn finish(&self) -> u64 {
        self.hash
    }

    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.hash^=byte as u64;
            self.hash=self.hash.wrapping_mul(random() as u64);
        }
    }
}
impl Default for MyHasher {
    fn default() -> Self {
        Self{hash:0}
    }
}
type MyBuildHasher = BuildHasherDefault<MyHasher>;