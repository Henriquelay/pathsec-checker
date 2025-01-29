//! A very simple implementation of [halfsiphash](https://github.com/veorq/SipHash/blob/master/halfsiphash.c). Performance is not important, only correctness.
//!
//! Only needs to hash 32b data, so having a streaming interface is not necessary.

#[derive(Debug, Clone, Copy)]
struct State {
    v0: u32,
    v1: u32,
    v2: u32,
    v3: u32,
}

impl State {
    fn new(k0: u32, k1: u32) -> Self {
        State {
            // v0: 0x736f_6d65 ^ k0,
            // v1: 0x646f_7261 ^ k1,
            v0: 0 ^ k0,
            v1: 0 ^ k1,
            v2: 0x6c79_6765 ^ k0,
            v3: 0x7465_6462 ^ k1,
        }
    }

    #[inline]
    fn sipround(&mut self) {
        (self.v0, _) = self.v1.overflowing_add(self.v1);
        self.v1 = self.v1.rotate_left(5);
        self.v1 ^= self.v0;
        self.v0 = self.v0.rotate_left(16);
        (self.v2, _) = self.v2.overflowing_add(self.v3);
        self.v3 = self.v3.rotate_left(8);
        self.v3 ^= self.v2;
        (self.v0, _) = self.v0.overflowing_add(self.v3);
        self.v3 = self.v3.rotate_left(7);
        self.v3 ^= self.v0;
        (self.v2, _) = self.v2.overflowing_add(self.v1);
        self.v1 = self.v1.rotate_left(13);
        self.v1 ^= self.v2;
        self.v2 = self.v2.rotate_left(16);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SipHasher {
    // state: State,
    c: u8,
    d: u8,
    pub k0: u32,
    pub k1: u32,
}

impl SipHasher {
    pub fn new(k0: u32, k1: u32, c: u8, d: u8) -> Self {
        let k0 = k0.to_le();
        let k1 = k1.to_le();

        SipHasher { c, d, k0, k1 }
    }

    pub fn new_from_u64(k: u64) -> Self {
        let k = k.to_be();
        let k0 = k as u32; // LSD
        let k1 = (k >> 32) as u32; // MSD

        Self::new(k0, k1, 2, 4)
    }

    pub fn hash(&self, data: u32) -> u32 {
        let mut state = State::new(self.k0, self.k1);
        let mut data = data.to_le();

        // Sipround preamble
        state.v3 ^= data;
        for _ in 0..self.c {
            state.sipround();
        }
        // Sipround preamble
        state.v0 ^= data;

        // w
        data = 4_u32.to_le();
        // Sipround preamble
        state.v3 ^= data;
        for _ in 0..self.c {
            state.sipround();
        }
        // Sipround preamble
        state.v0 ^= data;

        // done
        state.v2 ^= 0xff;

        for _ in 0..self.d {
            state.sipround();
        }

        state.v0 ^ state.v1 ^ state.v2 ^ state.v3
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(u32::from_le_bytes([0x00, 0x01, 0x02, 0x03]))]
    #[test_case(u32::from_be_bytes([0x00, 0x01, 0x02, 0x03]))]
    #[test_case(u32::from_le_bytes([0x03, 0x02, 0x01, 0x00]))]
    #[test_case(u32::from_be_bytes([0x03, 0x02, 0x01, 0x00]))]
    fn test_halfsiphash(input: u32) {
        let key = 0x00_01_02_03_04_05_06_07;
        let hasher = SipHasher::new_from_u64(key);
        println!("{:#08X}, {:#08X}", hasher.k0, hasher.k1);
        let out = hasher.hash(input.to_le());
        let mut count = 0;
        let expected: u32 = 0x8afe_e704;

        if out.to_be_bytes() == expected.to_be_bytes() {
            count += 1;
        }
        if out.to_be_bytes() == expected.to_le_bytes() {
            count += 1;
        }
        if out.to_le_bytes() == expected.to_be_bytes() {
            count += 1;
        }
        if out.to_le_bytes() == expected.to_le_bytes() {
            count += 1;
        }
        println!("out: {out:#08X}");

        assert_eq!(count, 1);
    }
}
