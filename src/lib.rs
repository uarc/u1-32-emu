#![feature(inclusive_range)]

use std::ops::RangeInclusive;

struct BusState {
    /// Bus handler address
    handler: Vec<u32>,
    /// Bus interrupt enable
    enabled: bool,
    /// Bus set
    set: bool,
}

pub struct Core {
    /// Interrupt enable flag
    int_enable: bool,
    /// Default handler address
    handler: u32,

    /// Data Stack (dstack)
    dstack: Vec<u32>,
    /// Bus interrupt states (ifile)
    ifile: Vec<u32>,
    /// Call Stack (cstack)
    cstack: Vec<u32>,
    /// Loop Stack (lstack)
    lstack: Vec<RangeInclusive<u32>>,
}

