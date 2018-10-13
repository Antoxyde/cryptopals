use libs::aes_utils::to_blocks;
use libs::xor::key_cycling_xor;
//https://csrc.nist.gov/csrc/media/publications/fips/197/final/documents/fips-197.pdf

//probably the most unoptimized implementation of aes you've ever seen, but it was a great way to understand (a bit) of aes's internals

const NB: u8 = 4;

const S_BOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

const INV_S_BOX: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
];

/*
fn rcon(mut i: u8) -> Vec<u8> {
    let mut r: Vec<u8> = vec![0x02, 0x00, 0x00, 0x00];

    if i == 1 {
        r[0] = 0x01;
    } else if i > 1 {
        r[0] = 0x02;
        i -= 1;
        while i - 1 > 0 {
            r[0] = gf256_mul(r[0], 0x02);
            i -= 1;
        }
    }

    return r;
}

fn gf256_add(a: u8, b: u8) -> u8 {
    println!("[debug] in gf256_add");
    a ^ b
}

fn gf256_sub(a: u8, b: u8) -> u8 {
    println!("[debug] in gf256_sub");
    a ^ b
}
*/

const RCON: [u8; 256] = [
    0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36, 0x6c, 0xd8, 0xab, 0x4d, 0x9a,
    0x2f, 0x5e, 0xbc, 0x63, 0xc6, 0x97, 0x35, 0x6a, 0xd4, 0xb3, 0x7d, 0xfa, 0xef, 0xc5, 0x91, 0x39,
    0x72, 0xe4, 0xd3, 0xbd, 0x61, 0xc2, 0x9f, 0x25, 0x4a, 0x94, 0x33, 0x66, 0xcc, 0x83, 0x1d, 0x3a,
    0x74, 0xe8, 0xcb, 0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36, 0x6c, 0xd8,
    0xab, 0x4d, 0x9a, 0x2f, 0x5e, 0xbc, 0x63, 0xc6, 0x97, 0x35, 0x6a, 0xd4, 0xb3, 0x7d, 0xfa, 0xef,
    0xc5, 0x91, 0x39, 0x72, 0xe4, 0xd3, 0xbd, 0x61, 0xc2, 0x9f, 0x25, 0x4a, 0x94, 0x33, 0x66, 0xcc,
    0x83, 0x1d, 0x3a, 0x74, 0xe8, 0xcb, 0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b,
    0x36, 0x6c, 0xd8, 0xab, 0x4d, 0x9a, 0x2f, 0x5e, 0xbc, 0x63, 0xc6, 0x97, 0x35, 0x6a, 0xd4, 0xb3,
    0x7d, 0xfa, 0xef, 0xc5, 0x91, 0x39, 0x72, 0xe4, 0xd3, 0xbd, 0x61, 0xc2, 0x9f, 0x25, 0x4a, 0x94,
    0x33, 0x66, 0xcc, 0x83, 0x1d, 0x3a, 0x74, 0xe8, 0xcb, 0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20,
    0x40, 0x80, 0x1b, 0x36, 0x6c, 0xd8, 0xab, 0x4d, 0x9a, 0x2f, 0x5e, 0xbc, 0x63, 0xc6, 0x97, 0x35,
    0x6a, 0xd4, 0xb3, 0x7d, 0xfa, 0xef, 0xc5, 0x91, 0x39, 0x72, 0xe4, 0xd3, 0xbd, 0x61, 0xc2, 0x9f,
    0x25, 0x4a, 0x94, 0x33, 0x66, 0xcc, 0x83, 0x1d, 0x3a, 0x74, 0xe8, 0xcb, 0x8d, 0x01, 0x02, 0x04,
    0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36, 0x6c, 0xd8, 0xab, 0x4d, 0x9a, 0x2f, 0x5e, 0xbc, 0x63,
    0xc6, 0x97, 0x35, 0x6a, 0xd4, 0xb3, 0x7d, 0xfa, 0xef, 0xc5, 0x91, 0x39, 0x72, 0xe4, 0xd3, 0xbd,
    0x61, 0xc2, 0x9f, 0x25, 0x4a, 0x94, 0x33, 0x66, 0xcc, 0x83, 0x1d, 0x3a, 0x74, 0xe8, 0xcb, 0x8d,
];

#[derive(PartialEq, Copy, Clone)]
pub enum OperationMode {
    ECB,
    CBC {iv: [u8; 16]},
}

pub struct AES {
    key: Vec<u8>,
    nr: u8,
    mode: OperationMode,
}

impl AES {
    //https://en.wikipedia.org/wiki/Finite_field_arithmetic#Rijndael's_finite_field
    //Peasant's algorithm
    //modulo used is x8 + x4 + x3 + x + 1

    fn gf256_mul(mut a: u8, mut b: u8) -> u8 {
        let mut p: u8 = 0;
        let mut carry: u8;

        for _ in 0..8 {
            if a == 0 || b == 0 {
                break;
            }

            if b & 1 == 1 {
                p ^= a;
            }

            b >>= 1;
            carry = a & 128;

            a <<= 1;
            if carry == 128 {
                a ^= 0x1b;
            }
        }
        return p;
    }

    fn gf256_poly_add(a: &[u8; 4], b: &[u8; 4]) -> [u8; 4] {
        let mut c = [0u8; 4];
        c[0] = a[0] ^ b[0];
        c[1] = a[1] ^ b[1];
        c[2] = a[2] ^ b[2];
        c[3] = a[3] ^ b[3];
        return c;
    }

    #[allow(dead_code)]
    fn gf256_poly_mul(a: &[u8; 4], b: &[u8; 4]) -> [u8; 4] {
        let mut d = [0u8; 4];
        d[0] = AES::gf256_mul(a[0], b[0]) ^ AES::gf256_mul(a[3], b[1]) ^ AES::gf256_mul(a[2], b[2])
            ^ AES::gf256_mul(a[1], b[3]);

        d[1] = AES::gf256_mul(a[1], b[0]) ^ AES::gf256_mul(a[0], b[1]) ^ AES::gf256_mul(a[3], b[2])
            ^ AES::gf256_mul(a[2], b[3]);

        d[2] = AES::gf256_mul(a[2], b[0]) ^ AES::gf256_mul(a[1], b[1]) ^ AES::gf256_mul(a[0], b[2])
            ^ AES::gf256_mul(a[3], b[3]);

        d[3] = AES::gf256_mul(a[3], b[0]) ^ AES::gf256_mul(a[2], b[1]) ^ AES::gf256_mul(a[1], b[2])
            ^ AES::gf256_mul(a[0], b[3]);
        return d;
    }


    //Place the first index at the end (aka cyclic permutation)
    fn rot_word(w: &mut [u8; 4]) {
        let t = w[0];
        w[0] = w[1];
        w[1] = w[2];
        w[2] = w[3];
        w[3] = t;
    }

    fn sub_word(w: &mut [u8; 4]) {
        for i in 0..4 {
            w[i] = S_BOX[w[i] as usize];
        }
    }

    fn sub_bytes(state: &mut [u8; 16]) {
        for i in 0..16 {
            state[i] = S_BOX[state[i] as usize];
        }
    }

    fn inv_sub_bytes(state: &mut [u8; 16]) {
        for i in 0..16 {
            state[i] = INV_S_BOX[state[i] as usize];
        }
    }

    fn add_round_key(key: &[u8], state: &mut [u8; 16], r: u8) {
        for i in 0..16 {
            state[i] ^= key[(4 * NB * r) as usize + i];
        }
    }

    fn inv_shift_rows(state: &mut [u8; 16]) {

        let mut temp = vec![0; 16];

        temp[0] = state[0];
        temp[1] = state[13];
        temp[2] = state[10];
        temp[3] = state[7];

        temp[4] = state[4];
        temp[5] = state[1];
        temp[6] = state[14];
        temp[7] = state[11];

        temp[8] = state[8];
        temp[9] = state[5];
        temp[10] = state[2];
        temp[11] = state[15];

        temp[12] = state[12];
        temp[13] = state[9];
        temp[14] = state[6];
        temp[15] = state[3];

        for i in 0..16 {
            state[i] = temp[i];
        }

    }

    fn shift_rows(state: &mut [u8; 16]) {
        let mut temp = vec![0; 16];
        temp[0] = state[0];
        temp[1] = state[5];
        temp[2] = state[10];
        temp[3] = state[15];

        temp[4] = state[4];
        temp[5] = state[9];
        temp[6] = state[14];
        temp[7] = state[3];

        temp[8] = state[8];
        temp[9] = state[13];
        temp[10] = state[2];
        temp[11] = state[7];

        temp[12] = state[12];
        temp[13] = state[1];
        temp[14] = state[6];
        temp[15] = state[11];

        for i in 0..16 {
            state[i] = temp[i];
        }
    }

    fn mix_columns(state: &mut [u8; 16]) {
        for i in 0..4 {
            let b0 = state[4 * i + 0];
            let b1 = state[4 * i + 1];
            let b2 = state[4 * i + 2];
            let b3 = state[4 * i + 3];

            state[4 * i + 0] = AES::gf256_mul(2, b0) ^ AES::gf256_mul(3, b1) ^ b2 ^ b3;
            state[4 * i + 1] = b0 ^ AES::gf256_mul(2, b1) ^ AES::gf256_mul(3, b2) ^ b3;
            state[4 * i + 2] = b0 ^ b1 ^ AES::gf256_mul(2, b2) ^ AES::gf256_mul(3, b3);
            state[4 * i + 3] = AES::gf256_mul(3, b0) ^ b1 ^ b2 ^ AES::gf256_mul(2, b3);
        }
    }

    fn inv_mix_columns(state: &mut [u8; 16]) {
        for i in 0..4 {
            let b0 = state[4 * i + 0];
            let b1 = state[4 * i + 1];
            let b2 = state[4 * i + 2];
            let b3 = state[4 * i + 3];

            state[4 * i + 0] = AES::gf256_mul(14, b0) ^ AES::gf256_mul(11, b1) ^ AES::gf256_mul(13, b2) ^ AES::gf256_mul(9, b3);
            state[4 * i + 1] = AES::gf256_mul(9, b0) ^ AES::gf256_mul(14, b1) ^ AES::gf256_mul(11, b2) ^ AES::gf256_mul(13, b3);
            state[4 * i + 2] = AES::gf256_mul(13, b0) ^ AES::gf256_mul(9, b1) ^ AES::gf256_mul(14, b2) ^ AES::gf256_mul(11, b3);
            state[4 * i + 3] = AES::gf256_mul(11, b0) ^ AES::gf256_mul(13, b1) ^ AES::gf256_mul(9, b2) ^ AES::gf256_mul(14, b3);
        }
    }


        //https://en.wikipedia.org/wiki/Rijndael_key_schedule
        //http://www.samiam.org/key-schedule.html
        fn key_expansion(k: &[u8], nk: u8, nr: u8) -> Vec<u8> {
            let mut tmp: [u8; 4] = [0, 0, 0, 0];

            let len = NB * (nr + 1);
            let mut w: Vec<u8> = Vec::new();

            //init the vector
            for _ in 0..len * 4 {
                w.push(0);
            }

            for i in 0..nk {
                w[(4 * i + 0) as usize] = k[(4 * i + 0) as usize];
                w[(4 * i + 1) as usize] = k[(4 * i + 1) as usize];
                w[(4 * i + 2) as usize] = k[(4 * i + 2) as usize];
                w[(4 * i + 3) as usize] = k[(4 * i + 3) as usize];
            }

            for i in nk..len {
                tmp[0] = w[(4 * (i - 1) + 0) as usize];
                tmp[1] = w[(4 * (i - 1) + 1) as usize];
                tmp[2] = w[(4 * (i - 1) + 2) as usize];
                tmp[3] = w[(4 * (i - 1) + 3) as usize];

                if i % nk == 0 {
                    AES::rot_word(&mut tmp);
                    AES::sub_word(&mut tmp);
                    let tr = [RCON[(i / nk) as usize], 0, 0, 0];
                    tmp = AES::gf256_poly_add(&tmp, &tr);
                } else if nk > 6 && i % nk == 4 {
                    AES::sub_word(&mut tmp);
                }

                w[(4 * i + 0) as usize] = w[(4 * (i - nk) + 0) as usize] ^ tmp[0];
                w[(4 * i + 1) as usize] = w[(4 * (i - nk) + 1) as usize] ^ tmp[1];
                w[(4 * i + 2) as usize] = w[(4 * (i - nk) + 2) as usize] ^ tmp[2];
                w[(4 * i + 3) as usize] = w[(4 * (i - nk) + 3) as usize] ^ tmp[3];
            }

            return w;
        }

    fn cipher(&mut self, state: &mut [u8; 16]) {
        let nr = self.nr;

        AES::add_round_key(&self.key, state, 0);

        for r in 1..nr {
            AES::sub_bytes(state);
            AES::shift_rows(state);
            AES::mix_columns(state);
            AES::add_round_key(&self.key, state, r);
        }

        AES::sub_bytes(state);
        AES::shift_rows(state);
        AES::add_round_key(&self.key, state, nr);
    }


    fn inv_cipher(&mut self, state: &mut [u8; 16]) {
        let nr = self.nr;

        AES::add_round_key(&self.key, state, nr);
        for r in (1..nr).rev() {
            AES::inv_shift_rows(state);
            AES::inv_sub_bytes(state);
            AES::add_round_key(&self.key, state, r);
            AES::inv_mix_columns(state);
        }

        AES::inv_shift_rows(state);
        AES::inv_sub_bytes(state);
        AES::add_round_key(&self.key, state, 0);

    }

    fn aes_ecb_decrypt(&mut self, input: &[u8]) -> Vec<u8> {
        let blocks: Vec<[u8; 16]> = to_blocks(input);
        let mut result: Vec<u8> = Vec::new();

        for mut block in blocks {
            self.inv_cipher(&mut block);
            for byte in block.iter() {
                result.push(*byte);
            }
        }

        return result;
    }

    fn aes_ecb_encrypt(&mut self, input: &[u8]) -> Vec<u8> {
        let blocks: Vec<[u8; 16]> = to_blocks(input);
        let mut result: Vec<u8> = Vec::new();

        for mut block in blocks {
            self.cipher(&mut block);
            for byte in block.iter() {
                result.push(*byte);
            }
        }

        return result;
    }

    fn aes_cbc_decrypt(&mut self, input: &[u8]) -> Vec<u8> {
        let blocks: Vec<[u8; 16]> = to_blocks(input);
        let mut result: Vec<u8> = Vec::new();
        let mut old_block = match self.mode {
            OperationMode::CBC { iv } => iv,
            _ => panic!("Wtf"),
        };

        for block in blocks {
            let mut current_block: [u8; 16] = block.clone(); //Save the current CT to decrypt the next block (xor)

            self.inv_cipher(&mut current_block);

            let temp_plain_block: Vec<u8> = key_cycling_xor(&current_block, &old_block);
            let mut plain_block = [0u8; 16];
            plain_block.copy_from_slice(&temp_plain_block[0..16]);

            for byte in plain_block.iter() {
                result.push(*byte);
            }

            old_block = block;
        }



        return result;
    }

    fn aes_cbc_encrypt(&mut self, input: &[u8]) -> Vec<u8> {
        let blocks: Vec<[u8; 16]> = to_blocks(input);
        let mut result: Vec<u8> = Vec::new();
        let mut old_block = match self.mode {
            OperationMode::CBC { iv } => iv,
            _ => panic!("Wtf"),
        };

        for block in blocks {
            let temp_xored_block: Vec<u8> = key_cycling_xor(&block, &old_block);
            let mut xored_block = [0u8; 16];

            xored_block.copy_from_slice(&temp_xored_block[0..16]);

            self.cipher(&mut xored_block);

            old_block = xored_block;

            for byte in xored_block.iter() {
                result.push(*byte);
            }
        }

        return result;
    }

    pub fn decrypt(&mut self, input: &[u8]) -> Vec<u8> {
        let result: Vec<u8> =  match self.mode {
            OperationMode::ECB => self.aes_ecb_decrypt(input),
            OperationMode::CBC{ .. } => self.aes_cbc_decrypt(input),
        };
        return result;
    }

    pub fn encrypt(&mut self, input: &[u8]) -> Vec<u8> {

        let result: Vec<u8> =  match self.mode {
            OperationMode::ECB => self.aes_ecb_encrypt(input),
            OperationMode::CBC{ .. } => self.aes_cbc_encrypt(input),
        };

        return result;
    }

    pub fn new(key: &[u8], mode: OperationMode) -> AES {

        let (nk, nr) = match key.len() {
            16 => Ok((4, 10)),
            24 => Ok((6, 12)),
            32 => Ok((8, 14)),
            _ => Err(key.len()),
        }.expect("Wrong key length");

        AES {
            key: AES::key_expansion(key, nk, nr),
            nr: nr,
            mode: mode,
        }
    }

}
