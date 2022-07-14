
use generic::xor::fixed_xor;
use symmetric::aes_core::{S_BOX,gf256_mul, key_expansion, cipher};
use symmetric::aes_utils::reverse_key_schedule_128;


#[derive(PartialEq, Copy, Clone)]
pub struct AesFault {
	pub index: u8,
	pub value: u8,
	pub round: u8,
}

fn get_possible_z(diff: u8, possible_z: &[u8], coeff: u8) -> Vec<u8> {

	let mut ret: Vec<u8> = Vec::new();

	let previous_z: Vec<u8> = match possible_z.len() {
		0 => (0..255 as u8).collect(),
		_ => possible_z.to_owned(),
	};

	for z in previous_z {
		for y in 0..255 as u8 {
			if !ret.contains(&z) && S_BOX[y as usize] ^ S_BOX[ (gf256_mul(coeff, z) ^ y) as usize] == diff {
				ret.push(z);
			}
		}
	}

	ret
}


fn get_ys_from_z(diff: u8, zs: &[u8], coeff:u8, ys: &[u8]) -> Vec<u8> {

	let mut ret = Vec::new();

	let previous_y: Vec<u8> = match ys.len() {
		0 => (0..255 as u8).collect(),
		_ => ys.to_owned(),
	};

	for y in previous_y {
		for &z in zs {
			if S_BOX[y as usize] ^ S_BOX[ (gf256_mul(coeff, z) ^ y) as usize] == diff {
				ret.push(y);
			}
		}
	}

	 ret
}


fn get_k_from_y(ys: &[u8], o0: u8) -> Vec<u8> {

	// Recover k10 form the given y0
	let mut k = Vec::new();

	for &y in ys {
		k.push(S_BOX[y as usize] ^ o0);
	}

	k
}



pub fn recover_quarter_k10(good : [u8; 16], faulteds: &[[u8; 16]], offset: [usize; 4], coeffs: [u8; 4]) -> (Vec<u8>, Vec<u8>,Vec<u8>,Vec<u8>) {
	// good : the correctly encrypted ciphertext
	// faulteds : a vector of faulty ciphertexts
	// offset : offset of the k10 to recover (eg [0,7,10,13] if the fault was in index 0)
	//
	// Return all the possibility for k10.


	let (mut y0, mut y1, mut y2, mut y3) = (Vec::new() ,Vec::new(),Vec::new(),Vec::new());

	for faulted in faulteds {

		let differential = fixed_xor(&good, faulted);

		if differential[offset[0]] != 0 {

			let mut possible_z = Vec::new();

			possible_z = get_possible_z(differential[offset[0]], &possible_z, coeffs[0]);
			possible_z = get_possible_z(differential[offset[1]], &possible_z, coeffs[1]);
			possible_z = get_possible_z(differential[offset[2]], &possible_z, coeffs[2]);
			possible_z = get_possible_z(differential[offset[3]], &possible_z, coeffs[3]);

			y0 = get_ys_from_z(differential[offset[0]], &possible_z, coeffs[0], &y0);
			y1 = get_ys_from_z(differential[offset[1]], &possible_z, coeffs[1], &y1);
			y2 = get_ys_from_z(differential[offset[2]], &possible_z, coeffs[2], &y2);
			y3 = get_ys_from_z(differential[offset[3]], &possible_z, coeffs[3], &y3);

		}
	}

    (get_k_from_y(&y0, good[offset[0]]), get_k_from_y(&y1, good[offset[1]]), get_k_from_y(&y2, good[offset[2]]),get_k_from_y(&y3, good[offset[3]]))
}


fn bf_possible_keys(plaintext: [u8; 16], good: [u8; 16], possible_keys : &[&Vec<u8>]) -> Option<Vec<u8>> {

	// Macro for cartesian product in rust are freaking hard so f*ck it
	// TODO : a macro like  bf![k0, k1...kn]; which expand to something like that ?

	for &k0 in possible_keys[0] {
		for &k1 in possible_keys[1] {
			for &k2 in possible_keys[2] {
				for &k3 in possible_keys[3] {
					for &k4 in possible_keys[4] {
						for &k5 in possible_keys[5] {
							for &k6 in possible_keys[6] {
								for &k7 in possible_keys[7] {
									for &k8 in possible_keys[8] {
										for &k9 in possible_keys[9] {
											for &k10 in possible_keys[10] {
												for &k11 in possible_keys[11] {
													for &k12 in possible_keys[12] {
														for &k13 in possible_keys[13] {
															for &k14 in possible_keys[14] {
																for &k15 in possible_keys[15] {

																	let pk =  [k0, k1, k2, k3, k4, k5, k6, k7, k8, k9, k10, k11, k12, k13, k14, k15];
																	let k = key_expansion(&reverse_key_schedule_128(&pk), 4, 10);
																	let mut pt = plaintext;

																	cipher(&k, &mut pt, 10, AesFault{index:0, value:0, round:9});

																	if pt == good {

																		return Some(Vec::from(&k[0..16]));
																	}
																}
															}
														}
													}
												}
											}
										}
									}
								}
							}
						}
					}
				}
			}
		}
	}

	None
}


pub  fn recover_aes_key_r9_injection(plaintext: [u8; 16], good : [u8; 16], faulteds: &[[u8; 16]]) -> Option<Vec<u8>> {
	// Return the aes key if given enough faulty ciphertexts
	// If the round key is fully determined, there is no bruteforce at all.
	// Otherwise we BF the possibilites  using a known pt/ct pair.

	let (k0, k7, k10, k13) = recover_quarter_k10(good, faulteds, [0, 7, 10, 13], [2, 3, 1, 1]);
	let (k1, k4, k11, k14) = recover_quarter_k10(good, faulteds, [1, 4, 11, 14], [1, 2, 3, 1]);
	let (k2, k5, k8, k15) = recover_quarter_k10(good, faulteds, [2, 5, 8, 15], [1, 1, 2, 3]);
	let (k3, k6, k9, k12) = recover_quarter_k10(good, faulteds, [3, 6, 9, 12], [3, 1, 1, 2]);


	let possibles_keys = &[&k0, &k1, &k2, &k3, &k4, &k5, &k6, &k7, &k8, &k9, &k10, &k11, &k12, &k13, &k14, &k15];

	let possibilities_by_k: Vec<usize> = possibles_keys.iter().map(|x| x.len()).collect();

	println!("Number of possibilites by key indice (1=determined, 0=absolutely free, x=x possibilities): {:?}", possibilities_by_k);

	bf_possible_keys(plaintext, good,  possibles_keys)

}
