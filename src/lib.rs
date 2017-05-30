#![feature(inclusive_range)]

use std::ops::RangeInclusive;

#[derive(Debug, Clone, Default)]
struct BusState {
    /// Bus handler address
    handler: Vec<usize>,
    /// Bus interrupt enable
    enabled: bool,
    /// Bus set
    set: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Core {
    /// Interrupt enable flag
    int_enable: bool,
    /// Default handler address
    handler: usize,
    /// Program Counter
    pc: usize,

    /// Data Stack (dstack)
    dstack: Vec<u32>,
    /// Bus interrupt states (ifile)
    ifile: Vec<BusState>,
    /// Call Stack (cstack)
    cstack: Vec<usize>,
    /// Loop Stack (lstack)
    lstack: Vec<RangeInclusive<usize>>,
}

