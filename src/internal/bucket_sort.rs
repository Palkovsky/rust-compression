//! rust-compression
//!
//! # Licensing
//! This Source Code is subject to the terms of the Mozilla Public License
//! version 2.0 (the "License"). You can obtain a copy of the License at
//! <http://mozilla.org/MPL/2.0/>.

use {MaxValue, MinValue};
use num_traits::{NumCast, cast};
use std::ops::{Add, Sub};


pub trait BucketSort {
    type Item;
    fn bucket_sort_by_key<
        K: Clone + Add + Sub<Output = K> + NumCast,
        F: Fn(&Self::Item) -> K,
    >(
        &self,
        key_selector: F,
        min: K,
        max: K,
    ) -> Vec<Self::Item>;

    fn bucket_sort_all_by_key<K, F>(&self, key_selector: F) -> Vec<Self::Item>
    where
        K: MaxValue + MinValue + Clone + Add + Sub<Output = K> + NumCast,
        F: Fn(&Self::Item) -> K,
    {
        self.bucket_sort_by_key(
            key_selector,
            MinValue::min_value(),
            MaxValue::max_value(),
        )
    }
}

impl<T: Clone> BucketSort for [T] {
    type Item = T;
    fn bucket_sort_by_key<
        K: Clone + Add + Sub<Output = K> + NumCast,
        F: Fn(&T) -> K,
    >(
        &self,
        key_selector: F,
        min: K,
        max: K,
    ) -> Vec<T> {
        let mut ret = self.to_vec();
        let mut bucket =
            vec![0; cast::<K, usize>(max - min.clone()).unwrap() + 2];

        for i in 0..self.len() {
            bucket[cast::<_, usize>(
                key_selector(&self[i]) - min.clone(),
            ).unwrap() + 1] += 1;
        }
        for i in 2..bucket.len() {
            bucket[i] += bucket[i - 1];
        }
        for i in self {
            let val = i.clone();
            let idx = cast::<_, usize>(key_selector(&val) - min.clone())
                .unwrap();
            ret[bucket[idx]] = val;
            bucket[idx] += 1;
        }
        ret
    }
}
