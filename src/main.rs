use std::{
    collections::HashSet,
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
    time::Duration,
};
use std::{thread, time};

const BLOOM_FILTER_SIZE: usize = 128;
const BLOOM_FILTER_ARITY: usize = 3;

struct BloomFilter {
    /// How many indices per inserted item are generated.
    arity: usize,
    /// The bytearray underpinning this data structure
    table: [bool; BLOOM_FILTER_SIZE],
}

impl BloomFilter {
    fn get_indices<T: Hash + Debug>(&self, item: T) -> Vec<u64> {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        let h = hasher.finish();
        let mut idx = Vec::with_capacity(self.arity);
        for i in 0..self.arity {
            idx.push(((i as u64 ^ h).wrapping_shl(i as u32)) % BLOOM_FILTER_SIZE as u64)
        }
        idx
    }
    fn insert<T: Hash + Debug>(&mut self, item: T) {
        let indices = self.get_indices(item);
        for i in indices {
            self.table[i as usize] = true;
        }
    }
    fn contains<T: Hash + Debug>(&self, item: T) -> bool {
        let indices = self.get_indices(item);
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

fn expensive_lookup<T: Hash + Debug + Eq>(store: &HashSet<T>, item: T) -> bool {
    let dur = time::Duration::from_millis(1);
    thread::sleep(dur);
    store.contains(&item)
}
fn retrieve_from_store<T: Hash + Debug + Eq>(
    bf: &BloomFilter,
    store: &HashSet<T>,
    item: T,
) -> bool {
    // 1. BF lookup
    if !bf.contains(&item) {
        return false;
    }
    // 2. expensive lookup
    expensive_lookup(store, item)
}

fn seed(store: &mut HashSet<u8>, bf: &mut BloomFilter) {
    (1u8..100).into_iter().for_each(|i| {
        store.insert(i);
        bf.insert(i);
    });
}

fn benchmark() -> (Duration, Duration, Duration) {
    let mut accept_time = Duration::ZERO;
    let mut reject_time = Duration::ZERO;
    let mut no_bf_time = Duration::ZERO;
    let (mut bf, mut store) = (BloomFilter::default(), HashSet::<u8>::new());
    println!("Created stores");
    seed(&mut store, &mut bf);
    println!("Done Seeding");
    (1u8..100)
        .zip(101u8..200)
        .zip(51u8..150)
        .into_iter()
        .for_each(|((c, n), e)| {
            {
                // Accepted -> These are in the bf
                let start = std::time::Instant::now();
                retrieve_from_store(&bf, &store, c);
                let end = std::time::Instant::now();
                accept_time = accept_time + (end - start);
            }
            {
                // Rejected: These are not in the bf and can be rejected quickly
                let start = std::time::Instant::now();
                retrieve_from_store(&bf, &store, n);
                let end = std::time::Instant::now();
                reject_time = reject_time + (end - start);
            }
            {
                // No bf: These always use the expensive lookup, without using a bf
                let start = std::time::Instant::now();
                expensive_lookup(&store, e);
                let end = std::time::Instant::now();
                no_bf_time = no_bf_time + (end - start);
            }
        });
    (accept_time / 100, reject_time / 100, no_bf_time / 100)
}

fn main() {
    let (a, r, w) = benchmark();
    println!(
        "Total time:\n  accept:     {:?}\n  reject:     {:?}\n  ---\n    avg: {:?}\n  without bf: {:?}",
        a,
        r,
        (a + r) / 2,
        w
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_tests() {
        let b = BloomFilter::default();
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
