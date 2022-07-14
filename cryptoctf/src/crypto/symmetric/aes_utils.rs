use rand;
use rand::Rng;

use symmetric::aes_core::*;
use symmetric::aes_dfa::AesFault;

pub fn inv_rot_word(w: &mut [u8; 4]) {
    let t = w[3];
    w[3] = w[2];
    w[2] = w[1];
    w[1] = w[0];
    w[0] = t;
}

pub fn inv_sub_word(w: &mut [u8; 4]) {
    for i in 0..4 {
        w[i] = INV_S_BOX[w[i] as usize];
    }
}

pub fn xor_word(w1: &[u8], w2: &[u8]) -> [u8; 4] {
    let mut w3 = [0u8; 4];
    for i in 0..4 {
        w3[i] = w1[i] ^ w2[i];
    }

    w3
}

pub fn gen_rnd_128_bits() -> [u8; 16] {
    let mut bits: [u8; 16] = [0; 16];
    let mut rng = rand::thread_rng();

    for x in &mut bits {
        *x = rng.gen::<u8>();
    }

    bits
}

pub fn aes_4round_encrypt(key: [u8; 16], data: [u8; 16]) -> [u8; 16] {
    let nr = 4;

    let subkey = key_expansion(&key, nr, 10);

    let mut state = data;


    add_round_key(&subkey, &mut state, 0);

    for r in 1..nr {
        round(&subkey, &mut state, r, AesFault{index:0,round:0,value:0});
    }

    final_round(&subkey, &mut state, nr);

    state

}

pub fn aes_4round_decrypt(key: [u8; 16], data: [u8; 16]) -> [u8; 16] {
    let nr = 4;
    let subkey = key_expansion(&key, nr, 10);

    let mut state = data;

    add_round_key(&subkey, &mut state, nr);

    for r in (1..nr).rev() {
        inv_round(&subkey, &mut state, r);
    }

    inv_final_round(&subkey, &mut state);

    state

}

pub fn reverse_key_schedule_4round(subkey: &[u8]) -> [u8; 16] {
    let nr = 4;
    let nk = 4;
    let nb = 4;

    let mut i = 0;

    let mut w = Vec::new();

    for _ in 0..nb*(nr+1)  {
        w.push([0,0,0,0]);
    }

    while i < nk {
        w[i] = [subkey[4 * i], subkey[4 * i + 1], subkey[4 * i + 2], subkey[4 * i + 3]];
        i += 1;
    }

    let mut j = nk;
    while j < nb*(nr+1) {
        if (j % nk) == 0 {
            w[j][0] = w[j - nk][0] ^ S_BOX[(w[j-1][1] ^ w[j-2][1]) as usize] ^ RCON[(nr - (j / nk) + 1) as usize];
            for i in 1..4 {
                w[j][i] = w[j - nk][i] ^ S_BOX[(w[j - 1][(i + 1) % 4] ^ w[j - 2][(i + 1) % 4]) as usize];
            }
        } else {
            w[j] = xor_word(&w[j - nk], &w[j - nk - 1]);
        }

        j += 1;
    }

    let mut result = [0u8; 16];

    for i in 16..20 {
        for j in 0..4 {
            result[(i-16) * 4 + j] = w[i][j];
        }
    }

    result

}

#[allow(clippy::identity_op)]
pub fn reverse_key_schedule_128(subkey: &[u8]) -> Vec<u8> {
    // Reverse key schedule of aes-128.
    // Inputs : subkey, the last round key
    // Return the first round key.

    let mut w1 = [0;4];
    let mut w2 = [0;4];
    let mut w3 = [0;4];
    let mut w4 = [0;4];

    w1[..4].clone_from_slice(&subkey[..4]);
    w2[..4].clone_from_slice(&subkey[4..(4 + 4)]);
    w3[..4].clone_from_slice(&subkey[8..(4 + 8)]);
    w4[..4].clone_from_slice(&subkey[12..(4 + 12)]);

    for i in 0..10 {

        let ww4 = xor_word(&w3 ,&w4);
        let ww3 = xor_word(&w2 ,&w3);
        let ww2 = xor_word(&w1 ,&w2);
        let mut cp_ww4 = ww4;

        rot_word(&mut cp_ww4);
        sub_word(&mut cp_ww4);
        cp_ww4[0] ^= RCON[10 - i as usize];
        let ww1 = xor_word(&cp_ww4 , &w1);

        w1 = ww1;
        w2 = ww2;
        w3 = ww3;
        w4 = ww4;

    }

    vec![w1, w2, w3, w4].concat()
}


use std::convert::AsMut;

fn clone_into_array<A, T>(slice: &[T]) -> A
    where A: Sized + Default + AsMut<[T]>,
          T: Clone
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}


pub fn concat_nonce_counter(n: &[u8], c: u64) -> [u8; 16] {

    let mut result = Vec::from(n);
    let c_as_u8: [u8; 8] = unsafe { std::mem::transmute::<u64, [u8; 8]>(c) };
    let r:Vec<u8> = c_as_u8.iter().copied().collect();
    result.extend_from_slice(&r);
    clone_into_array(&result)

}

pub fn to_aes_blocks(bytes: &[u8]) -> Vec<[u8; 16]> {
    assert!(bytes.len() % 16 == 0);

    let mut ret: Vec<[u8; 16]> = Vec::new();
    let pos: usize = bytes.len() / 16;

    for i in 0..pos {
        let mut arr = [0u8; 16];
        for j in 0..16 {
            arr[j] = bytes[i*16 + j];
        }
        ret.push(arr);
    }
    ret
}
