use crc::Crc;

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
    const ROUTE: [(Switch, Port); 10] = [
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

    let mut intermediate_lhashes = vec![EXPECTED_RESULT[0]];

    ROUTE.iter().rev().fold(EXPECTED_RESULT[0], |last_lhash, (switch, exit_port)| {
        let to_hash = exit_port ^ last_lhash ^ u32::from(switch.id);

        // Creating a new one for every iteration, on purpose
        let crc = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let result = crc.checksum(&to_hash.to_be_bytes());
        println!(
            "Switch: {id:#4x}, Port: {exit_port:?}, Previous LHASH: {last_lhash:#10x}. Current LHASH: {result:#10x}",
            id = switch.id
        );

        intermediate_lhashes.push(result);

        result
    });

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

// First is the initial hash, set by the edge, generated with `random()`
const EXPECTED_RESULT: [Hash; 7] = [
    // Doing XORs
    0xbaddc0de, 0xae91434c, 0x08c97f5f, 0xeff1aad2, 0x08040c89, 0xaa99ae2e, 0x98670972,
];

#[cfg(test)]
mod tests {
    use super::*;

    use crc::Algorithm;
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

    const EXPECTED_RESULT_NO_XORS: [Hash; 7] = [
        // Just lhash = last_lhash
        0xfdc18598, 0x7d059a69, 0x011adeda, 0xf1f44d84, 0x025103a3, 0x82c1f281, 0x4a4c6d52,
    ];

    const ROUTE: [(Switch, Port); 6] = [
        (CORE_SWITCHES[0], 0x1),
        (CORE_SWITCHES[1], 0x2),
        (CORE_SWITCHES[2], 0x2),
        (CORE_SWITCHES[3], 0x2),
        (CORE_SWITCHES[4], 0x2),
        (CORE_SWITCHES[5], 0x0),
    ];

    #[test]
    fn chained_crcs_multiple_constructor() {
        let mut intermediate_lhashes = vec![EXPECTED_RESULT_NO_XORS[0]];

        ROUTE.iter().fold(EXPECTED_RESULT_NO_XORS[0], |last_lhash, (switch, exit_port)| {
            let to_hash = last_lhash;

            // Creating a new one for every iteration, on purpose
            let crc = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
            let result = crc.checksum(&to_hash.to_be_bytes());
            println!(
                "Switch: {id:2X}, Port: {exit_port:?}, Previous LHASH: {last_lhash:8X}. Current LHASH: {result:8X}",
                id = switch.id
            );

            intermediate_lhashes.push(result);

            result
        });

        println!("Intermediate lhashes: {intermediate_lhashes:?}");

        assert_eq!(intermediate_lhashes, EXPECTED_RESULT_NO_XORS);
    }

    #[test]
    fn chained_crcs_single_constructor() {
        let mut intermediate_lhashes = vec![EXPECTED_RESULT_NO_XORS[0]];

        // Creating a single for all iterations, on purpose
        let crc = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        ROUTE.iter().fold(EXPECTED_RESULT_NO_XORS[0], |last_lhash, (switch, exit_port)| {
            let to_hash = last_lhash;

            let result = crc.checksum(&to_hash.to_be_bytes());
            println!(
                "Switch: {id:2X}, Port: {exit_port:?}, Previous LHASH: {last_lhash:8X}. Current LHASH: {result:8X}",
                id = switch.id
            );

            intermediate_lhashes.push(result);

            result
        });

        println!("Intermediate lhashes: {intermediate_lhashes:?}");

        assert_eq!(intermediate_lhashes, EXPECTED_RESULT_NO_XORS);
    }

    use crc::algorithm::*;

    #[test_case(&CRC_32_ISO_HDLC)]
    #[test_case(&CRC_32_AIXM => panics)]
    #[test_case(&CRC_32_AUTOSAR => panics)]
    #[test_case(&CRC_32_BASE91_D => panics)]
    #[test_case(&CRC_32_BZIP2 => panics)]
    #[test_case(&CRC_32_CD_ROM_EDC => panics)]
    #[test_case(&CRC_32_CKSUM => panics)]
    #[test_case(&CRC_32_ISCSI => panics)]
    #[test_case(&CRC_32_JAMCRC => panics)]
    #[test_case(&CRC_32_MEF => panics)]
    #[test_case(&CRC_32_MPEG_2 => panics)]
    #[test_case(&CRC_32_XFER => panics)]
    fn crc_modes(crc_mode: &'static Algorithm<u32>) {
        let mut intermediate_lhashes = vec![EXPECTED_RESULT_NO_XORS[0]];

        // Creating a single for all iterations, on purpose
        let crc = Crc::<u32>::new(crc_mode);
        ROUTE.iter().fold(intermediate_lhashes[0], |last_lhash, (switch, exit_port)| {
            let to_hash = last_lhash;

            let result = crc.checksum(&to_hash.to_be_bytes());
            println!(
                "Switch: {id:2X}, Port: {exit_port:?}, Previous LHASH: {last_lhash:8X}. Current LHASH: {result:8X}",
                id = switch.id
            );

            intermediate_lhashes.push(result);

            result
        });

        println!("Intermediate lhashes: {intermediate_lhashes:?}");

        assert_eq!(intermediate_lhashes, EXPECTED_RESULT_NO_XORS);
    }
}
