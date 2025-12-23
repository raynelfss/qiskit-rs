// This code is part of Qiskit Rust bindings.
//
// (C) Copyright IBM 2025
//
// This code is licensed under the Apache License, Version 2.0. You may
// obtain a copy of this license in the LICENSE.txt file in the root directory
// of this source tree or at http://www.apache.org/licenses/LICENSE-2.0.
//
// Any modifications or derivative works of this code must retain this
// copyright notice, and modified files need to carry a notice indicating
// that they have been altered from the originals.

use std::ptr::{null, null_mut};

use qiskit_sys::{
    QkGate, QkTarget, QkTargetEntry, qk_target_acquire_alignment, qk_target_copy, qk_target_dt,
    qk_target_entry_new, qk_target_free, qk_target_granularity, qk_target_min_length,
    qk_target_new, qk_target_set_acquire_alignment, qk_target_set_dt, qk_target_set_granularity,
    qk_target_set_min_length,
};
use smallvec::SmallVec;

#[derive(Debug)]
pub struct Target(*mut QkTarget);

impl Target {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_acquire_alignment(self, acquire_alignment: u32) -> Self {
        unsafe {
            qk_target_set_acquire_alignment(self.0, acquire_alignment);
        }
        self
    }

    pub fn with_dt(self, dt: f64) -> Self {
        unsafe {
            qk_target_set_dt(self.0, dt);
        }
        self
    }

    pub fn with_granularity(self, granularity: u32) -> Self {
        unsafe {
            qk_target_set_granularity(self.0, granularity);
        }
        self
    }

    pub fn with_min_length(self, dt: u32) -> Self {
        unsafe {
            qk_target_set_min_length(self.0, dt);
        }
        self
    }

    pub fn with_pulse_alignment(self, pulse_alignment: u32) -> Self {
        unsafe {
            qk_target_set_acquire_alignment(self.0, pulse_alignment);
        }
        self
    }

    pub fn acquire_alignment(&self) -> u32 {
        unsafe { qk_target_acquire_alignment(self.0) }
    }

    pub fn dt(&self) -> f64 {
        unsafe { qk_target_dt(self.0) }
    }

    pub fn granularity(&self) -> u32 {
        unsafe { qk_target_granularity(self.0) }
    }

    pub fn min_length(&self) -> u32 {
        unsafe { qk_target_min_length(self.0) }
    }

    pub fn pulse_alignment(&self) -> u32 {
        unsafe { qk_target_acquire_alignment(self.0) }
    }
}

impl Clone for Target {
    fn clone(&self) -> Self {
        let cloned = unsafe { qk_target_copy(self.0) };
        Self(cloned)
    }
}

impl Default for Target {
    fn default() -> Self {
        Self(unsafe { qk_target_new(0) })
    }
}

impl Drop for Target {
    fn drop(&mut self) {
        unsafe { qk_target_free(self.0) };
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Qargs {
    Global,
    Concrete(SmallVec<[u32; 2]>),
}

pub enum QargsRef<'a> {
    Global,
    Concrete(&'a [u32]),
}

impl From<&mut Qargs> for *mut u32 {
    fn from(val: &mut Qargs) -> Self {
        match val {
            Qargs::Global => null_mut(),
            Qargs::Concrete(small_vec) => small_vec.as_mut_ptr(),
        }
    }
}

impl From<&Qargs> for *const u32 {
    fn from(val: &Qargs) -> Self {
        match val {
            Qargs::Global => null(),
            Qargs::Concrete(small_vec) => small_vec.as_ptr(),
        }
    }
}

impl Qargs {
    pub fn as_ref(&self) -> QargsRef<'_> {
        match self {
            Qargs::Global => QargsRef::Global,
            Qargs::Concrete(small_vec) => QargsRef::Concrete(small_vec),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InstructionProperies {
    pub duration: f64,
    pub error: f64,
}

pub struct TargetEntry(*mut QkTargetEntry);

impl TargetEntry {
    pub fn new(operation: QkGate) -> Self {
        Self(unsafe { qk_target_entry_new(operation) })
    }
}
