#![deny(clippy::pedantic)]

mod crc;
mod siphash;

pub type Port = u32;
pub type Hash = u32;
pub type Route<'a> = &'a [(Switch, Port)];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Switch {
    id: u16,
}

const CORE_SWITCHES: [Switch; 10] = [
    Switch { id: 0x2b },
    Switch { id: 0x2d },
    Switch { id: 0x39 },
    Switch { id: 0x3f },
    Switch { id: 0x47 },
    Switch { id: 0x53 },
    Switch { id: 0x8d },
    Switch { id: 0xbd },
    Switch { id: 0xd7 },
    Switch { id: 0xf5 },
];

const ROUTE: Route = &[
    (CORE_SWITCHES[0], 0x2),
    (CORE_SWITCHES[1], 0x2),
    (CORE_SWITCHES[2], 0x2),
    (CORE_SWITCHES[3], 0x2),
    (CORE_SWITCHES[4], 0x2),
    (CORE_SWITCHES[5], 0x2),
    (CORE_SWITCHES[6], 0x2),
    (CORE_SWITCHES[7], 0x2),
    (CORE_SWITCHES[8], 0x2),
    (CORE_SWITCHES[9], 0x2),
];

fn main() {
    crc::check(ROUTE);
}
