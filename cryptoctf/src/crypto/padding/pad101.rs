pub fn pad101(input: &[u8], blocksize: usize) -> Vec<u8> {
    // blocksize is in bytes

   let mut r = input.len() % blocksize; 
   let mut padded = Vec::from(input);

   if r == 0 {
       r = blocksize;
    }

   if r == 1 {
       padded.push(0x81);
       padded
    } else {
        padded.push(0x80); 
        r -= 1;

        while r >= 1 {
            padded.push(0x00);
            r -= 1;
        }

        padded.push(0x01);

        padded
    }

}
