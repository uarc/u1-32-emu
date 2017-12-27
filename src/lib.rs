#![feature(inclusive_range)]

extern crate futures;
extern crate num;

use std::ops::RangeInclusive;
use std::collections::BTreeMap;
use num::{PrimInt, NumCast};
use futures::sync::mpsc::{Sender, Receiver};

#[derive(Debug, Clone, Default)]
struct U1Bus<W> {
    /// Bus handler address
    handler: W,
    /// Bus interrupt enable
    enabled: bool,
    /// Bus set
    set: bool,
    /// If the bus is already connected
    connected: bool,
}

#[derive(Debug, Clone)]
struct LElem<W> {
    program: RangeInclusive<W>,
    index: W,
}

#[derive(Debug, Clone)]
struct CElem<W> {
    return_address: W,
    dcs: [W; 2],
    carry: bool,
    overflow: bool,
    int_level: W,
}

#[derive(Debug, Clone, Default)]
pub struct Core<W: PrimInt> {
    /// Interrupt enable flag
    int_enable: bool,
    /// Program Counter
    pc: W,

    /// Data Counters [X, Y]
    dcs: [W; 2],
    /// Carry bit
    carry: bool,
    /// Overflow bit
    overflow: bool,
    /// Interrupt level
    int_level: W,
    /// Data Stack (dstack)
    dstack: Vec<W>,
    /// Bus interrupt states (ifile)
    ifile: BTreeMap<W, U1Bus<W>>,
    /// Call Stack (cstack)
    cstack: Vec<CElem<W>>,
    /// Loop Stack (lstack)
    lstack: Vec<LElem<W>>,
}

