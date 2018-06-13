pub fn pkcs7_unpad(data: &[u8], blocksize: usize) -> Result<Vec<u8>, &str> {

    if data.len() % blocksize != 0 {
        return Err("Invalid data size (not padded or wrong blocksize).");
    }

    let mut unpadded: Vec<u8> = data.clone().to_owned();

    let pad: u8 = unpadded[unpadded.len() - 1];

    if pad == 0 {
        return Ok(unpadded); //Data isn't padded
    } else if pad as usize > blocksize {
        return Err("Incorrect amount of padding.");
    }

    for _ in 0..pad {
        if  unpadded.pop().unwrap() != pad {
            return Err("Invalid padding encoutered.");
        }
    }

    return Ok(unpadded);
}

pub fn pkcs7_pad(data: &[u8], blocksize: usize) -> Vec<u8> {

    let pad: u8 = (blocksize - (data.len() % blocksize)) as u8;
    let mut padded: Vec<u8> = data.clone().to_owned();

    for _ in 0..pad {
        padded.push(pad);
    }

    return padded;
}
