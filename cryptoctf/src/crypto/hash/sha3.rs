use padding::pad101::pad101;
use utils::to_blocks;
use generic::xor::fixed_xor;

fn keccak_f(_input: &[u8]) -> Vec<u8>{
    //let state = Vec::from(input);
    //state
    Vec::new()
}

fn keccak(r: usize, input: &[u8], output_byte_length: usize) -> Vec<u8> {
    
    let mut state: Vec<u8> = vec![0u8; 200];

    // Pad step
    let input_blocks = to_blocks(&pad101(input, r), r);

    // Absorb step
    for block in input_blocks {
        let xored = fixed_xor(&block, &state[..r]);
        state[..r].copy_from_slice(&xored);
        state = keccak_f(&state);
    }

    // Squeeze step
    let mut result = Vec::new();

    while result.len() < output_byte_length {
        result.extend_from_slice(&state[..r]);
        state = keccak_f(&state);
    }
    
    result[..output_byte_length].to_vec()
}

pub fn sha3_256(input: &[u8]) -> Vec<u8> {
    keccak(1088, input, 32)
}

pub fn sha3_512(input: &[u8]) -> Vec<u8> {
    keccak(576, input, 64)
}
