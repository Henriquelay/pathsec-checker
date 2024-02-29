use crc::{Crc, Digest};

use std::collections::VecDeque;

#[deny(clippy::pedantic)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Switch {
    id: u16,
}

type Port = u32;

type Hash = u32;

fn main() {
    // A route for a packet going from h1 to h6, dependant on the topology
    // This is just the example captured in the wireshark snapshot
    const ROUTE: [(Switch, Port); 6] = [
        (CORE_SWITCHES[0], 0x1),
        (CORE_SWITCHES[1], 0x2),
        (CORE_SWITCHES[2], 0x2),
        (CORE_SWITCHES[3], 0x2),
        (CORE_SWITCHES[4], 0x2),
        (CORE_SWITCHES[5], 0x0),
    ];

    let mut intermediate_lhashes = vec![EXPECTED_RESULT[0]];

    for (i, (sw, exit_port)) in ROUTE.into_iter().enumerate() {
        let last_lhash = *intermediate_lhashes.last().unwrap();
        // Port is 9 bit wide, so we shift it to the left 23 times to pad zeroes to the right
        // let exit_port_padded = exit_port << (32 - 9);
        // The same happens with Switch id
        // let switch_id_padded = u32::from(sw.id) << (32 - 16);
        // let to_hash = exit_port_padded ^ last_lhash ^ switch_id_padded;
        let to_hash = last_lhash;

        // Creating a new one for every iteration, on purpose
        let crc = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let result = crc.checksum(&to_hash.to_be_bytes());
        println!(
            "[{i}] Switch: {id:2X}, Port: {exit_port:?}, Previous LHASH: {last_lhash:8X}. Current LHASH: {result:8X}",
            id = sw.id
        );
        intermediate_lhashes.push(result);
    }

    println!("Intermediate lhashes: {intermediate_lhashes:?}");

    assert_eq!(intermediate_lhashes, EXPECTED_RESULT);
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

// First is the initial hash, set by the edge
// const EXPECTED_RESULT: [Hash; 7] = [
//     0x61e8d6e7, 0xae91434c, 0x08c97f5f, 0xeff1aad2, 0x08040c89, 0xaa99ae2e, 0x98670972,
// ];
const EXPECTED_RESULT: [Hash; 7] = [
    0xfdc18598, 0x7d059a69, 0x011adeda, 0xf1f44d84, 0x025103a3, 0x82c1f281, 0x4a4c6d52,
];

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    // Values come from hashing each switche's SwitchId.
    #[test_case(0x0 => 0x2144df1c; "0")]
    #[test_case(0x1 => 0x5643ef8a; "1")]
    #[test_case(0x2b => 0x8df8265c; "0x2b")]
    #[test_case(0x2d => 0x649b8369; "0x2d")]
    #[test_case(0x39 => 0x7e415714; "0x39")]
    #[test_case(0x3f => 0x9722f221; "0x3f")]
    #[test_case(0x47 => 0xc9fc0b2f; "0x47")]
    #[test_case(0x53 => 0xd326df52; "0x53")]
    #[test_case(0x8d => 0xb24d2081; "0x8d")]
    #[test_case(0xbd => 0x9494102d; "0xbd")]
    #[test_case(0xd7 => 0x39f3986b; "0xd7")]
    #[test_case(0xf5 => 0xec93d98f; "0xf5")]
    fn crc_of_fixed_number(to_crc: u32) -> u32 {
        Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(&to_crc.to_be_bytes())
    }
}
