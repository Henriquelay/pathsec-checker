use crc::Crc;

const CRC_ALGO: &crc::Algorithm<u32> = &crc::CRC_32_ISO_HDLC;

use super::{Hash, Route};

pub fn check(route: Route) {
    // A route for a packet going from h1 to h6, dependant on the topology
    // This is just the example captured in the wireshark snapshot

    let mut intermediate_lhashes = vec![EXPECTED_RESULT[0]];

    route.iter().rev().fold(EXPECTED_RESULT[0], |last_lhash, (switch, exit_port)| {
        let to_hash = exit_port ^ last_lhash ^ u32::from(switch.id);

        // Creating a new one for every iteration, on purpose
        let crc = Crc::<u32>::new(CRC_ALGO);
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

// First is the initial hash, set by the edge, generated with `random()`
const EXPECTED_RESULT: [Hash; 7] = [
    // Doing XORs
    0xbadd_c0de,
    0xae91_434c,
    0x08c9_7f5f,
    0xeff1_aad2,
    0x0804_0c89,
    0xaa99_ae2e,
    0x9867_0972,
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Port, Switch, CORE_SWITCHES};

    use crc::Algorithm;
    use test_case::test_case;

    // Values come from hashing each switche's SwitchId.
    #[test_case(0x0 => 0x2144_df1c; "0")]
    #[test_case(0x1 => 0x5643_ef8a; "1")]
    #[test_case(0x2b => 0x8df8_265c; "0x2b")]
    #[test_case(0x2d => 0x649b_8369; "0x2d")]
    #[test_case(0x39 => 0x7e41_5714; "0x39")]
    #[test_case(0x3f => 0x9722_f221; "0x3f")]
    #[test_case(0x47 => 0xc9fc_0b2f; "0x47")]
    #[test_case(0x53 => 0xd326_df52; "0x53")]
    #[test_case(0x8d => 0xb24d_2081; "0x8d")]
    #[test_case(0xbd => 0x9494_102d; "0xbd")]
    #[test_case(0xd7 => 0x39f3_986b; "0xd7")]
    #[test_case(0xf5 => 0xec93_d98f; "0xf5")]
    fn crc_of_fixed_number(to_crc: u32) -> u32 {
        Crc::<u32>::new(CRC_ALGO).checksum(&to_crc.to_be_bytes())
    }

    const EXPECTED_RESULT_NO_XORS: [Hash; 7] = [
        // Just lhash = last_lhash
        0xfdc1_8598,
        0x7d05_9a69,
        0x011a_deda,
        0xf1f4_4d84,
        0x0251_03a3,
        0x82c1_f281,
        0x4a4c_6d52,
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
            let crc = Crc::<u32>::new(CRC_ALGO);
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
        let crc = Crc::<u32>::new(CRC_ALGO);
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
    #[test_case(&CRC_32_AIXM => panics "assertion `left == right` failed")]
    #[test_case(&CRC_32_AUTOSAR => panics "assertion `left == right` failed")]
    #[test_case(&CRC_32_BASE91_D => panics "assertion `left == right` failed")]
    #[test_case(&CRC_32_BZIP2 => panics "assertion `left == right` failed")]
    #[test_case(&CRC_32_CD_ROM_EDC => panics "assertion `left == right` failed")]
    #[test_case(&CRC_32_CKSUM => panics "assertion `left == right` failed")]
    #[test_case(&CRC_32_ISCSI => panics "assertion `left == right` failed")]
    #[test_case(&CRC_32_JAMCRC => panics "assertion `left == right` failed")]
    #[test_case(&CRC_32_MEF => panics "assertion `left == right` failed")]
    #[test_case(&CRC_32_MPEG_2 => panics "assertion `left == right` failed")]
    #[test_case(&CRC_32_XFER => panics "assertion `left == right` failed")]
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
