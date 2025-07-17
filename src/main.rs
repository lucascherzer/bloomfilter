use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};

const BLOOM_FILTER_SIZE: usize = 128;
const BLOOM_FILTER_ARITY: usize = 3;

struct BloomFilter {
    arity: usize,
    table: [bool; BLOOM_FILTER_SIZE],
}

impl BloomFilter {
    fn get_indices<T: Hash + Debug>(item: T) -> Vec<u32> {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        let h = hasher.finish() as u32;
        let d1 = (&h & 0b00_0000000000_0000000000_1111111111) % BLOOM_FILTER_SIZE as u32;
        let d2 = (&h & 0b00_0000000000_1111111111_0000000000) % BLOOM_FILTER_SIZE as u32;
        let d3 = (&h & 0b00_1111111111_0000000000_0000000000) % BLOOM_FILTER_SIZE as u32;
        println!("Generated points for {:?}: ({},{},{})", item, d1, d2, d3);
        vec![d1, d2, d3]
    }
    fn insert<T: Hash + Debug>(&mut self, item: T) {
        let indices = BloomFilter::get_indices(item);
        for i in indices {
            self.table[i as usize] = true;
        }
    }
    fn contains<T: Hash + Debug>(&self, item: T) -> bool {
        let indices = BloomFilter::get_indices(item);
        indices.into_iter().all(|i| self.table[i as usize])
    }
}

impl Default for BloomFilter {
    fn default() -> Self {
        BloomFilter {
            arity: BLOOM_FILTER_ARITY,
            table: [false; BLOOM_FILTER_SIZE],
        }
    }
}

fn expensive_lookup() {}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_tests() {
        let cases = vec![
            (vec![1], 1, true),
            (vec![1, 2], 2, true),
            (vec![], 1, false),
            (vec![2], 1, false),
        ];
        for c in cases {
            let mut bf = BloomFilter::default();
            for v in c.0 {
                bf.insert(v);
            }
            assert_eq!(bf.contains(c.1), c.2)
        }
    }
}
