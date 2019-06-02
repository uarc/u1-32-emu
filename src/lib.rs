#[derive(Debug, Clone)]
struct CElem {
    return_address: u32,
    registers: [u32; 2],
    carry: bool,
    overflow: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Core {
    /// Program Counter
    pc: u32,

    /// Data Counters [X, Y]
    dcs: [u32; 2],
    /// Carry bit
    carry: bool,
    /// Overflow bit
    overflow: bool,
    /// Data Stack (dstack)
    dstack: Vec<u32>,
    /// Call Stack (cstack)
    cstack: Vec<CElem>,

    /// Executed on an interrupt.
    interrupt_handler: u32,
    /// Determines whether an interrupt should be accepted.
    interrupt_enable: bool,

    /// Memory
    memory: Vec<u32>,
}

