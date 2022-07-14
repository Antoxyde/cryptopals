
use symmetric::aes_utils::{to_aes_blocks, concat_nonce_counter};
use symmetric::aes_core::*;
use generic::xor::key_cycling_xor;
use symmetric::aes_dfa::AesFault;


#[derive(PartialEq, Copy, Clone)]
pub enum OperationMode {
    ECB,
    CBC {iv: [u8; 16]},
    CTR {nonce: [u8; 8]},
}

pub struct AES {
    key: Vec<u8>,
    nr: u8,
    mode: OperationMode,
}

impl AES {
    
    fn aes_ctr_encrypt(&mut self, input: &[u8]) -> Vec<u8>{

        let nonce = match self.mode {
            OperationMode::CTR { nonce } => nonce,
            _ => panic!("Wtf"),
        };

        let mut counter = 0u64;
        let mut result: Vec<u8> = Vec::new();
       
        let mut key: [u8; 16] = concat_nonce_counter(&nonce, counter); 
        cipher(&self.key, &mut key, self.nr, AesFault{index:0,value:0,round:0});

        for i in 0..input.len() {

            if i % 16 == 0 && i > 0 {
                counter += 1;
                key = concat_nonce_counter(&nonce, counter);
                cipher(&self.key, &mut key, self.nr, AesFault{index:0,value:0,round:0});
            }

            result.push(input[i] ^ key[i % 16]);

        }

        result
    }

    fn aes_ecb_decrypt(&mut self, input: &[u8]) -> Vec<u8> {

        let blocks: Vec<[u8; 16]> = to_aes_blocks(input);
        let mut result: Vec<u8> = Vec::new();
        
        // Simply call inv_cipher for each block, and concatenate the results
        for mut block in blocks {
            inv_cipher(&self.key, &mut block, self.nr);
            for byte in block.iter() {
                result.push(*byte);
            }
        }

        result
    }

    fn aes_ecb_encrypt(&mut self, input: &[u8]) -> Vec<u8> {

        let blocks: Vec<[u8; 16]> = to_aes_blocks(input);
        let mut result: Vec<u8> = Vec::new();
    
        // Simply call cipher for each block, and concatenate the result
        for mut block in blocks {
            cipher(&self.key , &mut block, self.nr, AesFault{index:0,value:0,round:0});
            for byte in block.iter() {
                result.push(*byte);
            }
        }

        result
    }

    fn aes_cbc_decrypt(&mut self, input: &[u8]) -> Vec<u8> {

        let blocks: Vec<[u8; 16]> = to_aes_blocks(input);
        let mut result: Vec<u8> = Vec::new();

        let mut old_block = match self.mode {
            OperationMode::CBC { iv } => iv,
            _ => panic!("Wtf"),
        };

        for block in blocks {

            let mut current_block = block; //Save the current CT to decrypt the next block (xor)

            inv_cipher(&self.key, &mut current_block, self.nr);

            let temp_plain_block: Vec<u8> = key_cycling_xor(&current_block, &old_block);
            let mut plain_block = [0u8; 16];
            plain_block.copy_from_slice(&temp_plain_block[0..16]);

            for byte in plain_block.iter() {
                result.push(*byte);
            }

            old_block = block;
        }

        result
    }

    fn aes_cbc_encrypt(&mut self, input: &[u8]) -> Vec<u8> {

        let blocks: Vec<[u8; 16]> = to_aes_blocks(input);
        let mut result: Vec<u8> = Vec::new();

        let mut old_block = match self.mode {
            OperationMode::CBC { iv } => iv,
            _ => panic!("Wtf"),
        };

        for block in blocks {
            let temp_xored_block: Vec<u8> = key_cycling_xor(&block, &old_block);
            let mut xored_block = [0u8; 16];

            xored_block.copy_from_slice(&temp_xored_block[0..16]);

            cipher(&self.key, &mut xored_block, self.nr, AesFault{index:0,value:0,round:0});

            old_block = xored_block;

            for byte in xored_block.iter() {
                result.push(*byte);
            }
        }

        result
    }

    pub fn decrypt(&mut self, input: &[u8]) -> Vec<u8> {
        let result: Vec<u8> =  match self.mode {
            OperationMode::ECB => self.aes_ecb_decrypt(input),
            OperationMode::CBC{ .. } => self.aes_cbc_decrypt(input),
            OperationMode::CTR{ .. } => self.aes_ctr_encrypt(input),
        };
        result
    }

    pub fn encrypt(&mut self, input: &[u8]) -> Vec<u8> {

        let result: Vec<u8> =  match self.mode {
            OperationMode::ECB => self.aes_ecb_encrypt(input),
            OperationMode::CBC{ .. } => self.aes_cbc_encrypt(input),
            OperationMode::CTR{ .. } => self.aes_ctr_encrypt(input),
        };

        result
    }

    pub fn new(key: &[u8], mode: OperationMode) -> AES {

        let (nk, nr) = match key.len() {
            16 => Ok((4, 10)),
            24 => Ok((6, 12)),
            32 => Ok((8, 14)),
            _ => Err(key.len()),
        }.expect("Wrong key length");

        AES {
            key: key_expansion(key, nk, nr),
            nr,
            mode,
        }
    }
}
