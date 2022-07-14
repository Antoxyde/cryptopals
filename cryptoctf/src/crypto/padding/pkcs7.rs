pub fn pkcs7_unpad(data: &[u8], blocksize: usize) -> Result<Vec<u8>, &str> {

    if data.len() % blocksize != 0 {
        return Err("Invalid data size (not padded or wrong blocksize).");
    }

    let mut unpadded: Vec<u8> = Vec::from(data);
    let pad: u8 = unpadded[unpadded.len() - 1];

    if pad as usize > blocksize {
        return Err("Incorrect amount of padding.");
    }

    for _ in 0..pad {
        if  unpadded.pop().unwrap() != pad {
            return Err("Invalid padding encoutered.");
        }
    }

    Ok(unpadded)
}

pub fn pkcs7_pad(data: &[u8], blocksize: usize) -> Vec<u8> {

    let pad: u8 = (blocksize - (data.len() % blocksize)) as u8;
    vec![data, &vec![pad ; pad as usize]].concat()
}

pub fn pkcs7_oracle(data: &[u8], blocksize: usize) -> bool {
    pkcs7_unpad(data, blocksize).is_ok()
}
