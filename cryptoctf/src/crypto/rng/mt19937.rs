use std::num::Wrapping;

const N: usize = 624;
const W: usize = 32 ;
const M: usize = 397;
const U: usize = 11 ;
const S: usize = 7;
const T: usize = 15;
const L: usize = 18;

const A: Wrapping<u32> = Wrapping(2_567_483_615);
const C: Wrapping<u32>  = Wrapping(0xEFC6_0000);
const B: Wrapping<u32>  = Wrapping(0x9D2C_5680);
const F : Wrapping<u32> = Wrapping(1_812_433_253);

const LOWER_MASK: Wrapping<u32> = Wrapping(0x7fff_ffff);
const UPPER_MASK: Wrapping<u32> = Wrapping(0x8000_0000);

pub struct MT19937 {
    states: [Wrapping<u32>; N],
    index: usize,
}

impl MT19937 {

    pub fn new(seed: u32) -> Self {
        let mut mt = MT19937 { states: [Wrapping(0); N], index:N};

        mt.states[0] = Wrapping(seed);

        for i in 1..(N-1) {
            mt.states[i] = F  * (mt.states[i - 1] ^ ( mt.states[i - 1] >> (W - 2) )) + Wrapping(i as u32);
        }

        mt
    }

    pub fn rnd(&mut self) -> u32 {

        if self.index >= N {
            self.twist();
        }

        let mut y = self.states[self.index];

        y ^= y >> U;
        y ^= (y << S) & B;
        y ^= (y << T) & C;
        y ^= y >> L;

        self.index += 1;

        y.0
    }

    fn twist(&mut self) {

        for i in 0..(N-1) {
            let x: Wrapping<u32>  = (self.states[i] & UPPER_MASK) + (self.states[(i+1) % N] & LOWER_MASK);
            let mut x_a: Wrapping<u32> = x >> 1;

            if (x.0 % 2) != 0 {
                x_a ^= A;
            }

            self.states[i] = self.states[(i + M) % N] ^ x_a;

        }

        self.index = 0;
    }

}
