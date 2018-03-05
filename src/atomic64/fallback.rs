// Copyright 2014 The Prometheus Authors
// Copyright 2018 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

use super::Atomic;
use spin::RwLock;
use std::ops::AddAssign;

pub struct RwlockAtomic<T: AddAssign + Copy> {
    inner: RwLock<T>,
}

impl<T: AddAssign + Copy> Atomic<T> for RwlockAtomic<T> {
    fn new(val: T) -> Self {
        RwlockAtomic {
            inner: RwLock::new(val),
        }
    }

    #[inline]
    fn set(&self, val: T) {
        *self.inner.write() = val;
    }

    #[inline]
    fn get(&self) -> T {
        *self.inner.read()
    }

    #[inline]
    fn inc_by(&self, delta: T) {
        *self.inner.write() += delta;
    }
}

pub type AtomicF64 = RwlockAtomic<f64>;

pub type AtomicU64 = RwlockAtomic<u64>;
