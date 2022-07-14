pub struct RC4 {
    s: Vec<u8>,
    i: usize,
    j: usize,
}

impl RC4 {

    pub fn new(key: &[u8]) -> Self {

        let mut s: Vec<u8> = Vec::new();

        for i in 0..256 {
            s.push(i as u8);
        }

        let mut j: usize = 0;

        for i in 0..256 {
            j = (j  + s[i] as usize + key[i % key.len()] as usize) % 256;

            s.swap(i,j);

        }

        RC4 { s , i:0, j:0 }
    }

    pub fn encrypt(&mut self, data: &[u8]) -> Vec<u8> {
        data.iter()
            .zip(self)
            .map(|(a,b)| a^b)
            .collect()
    }
}

impl Iterator for RC4 {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {

        self.i = (self.i + 1) % 256;
        self.j = (self.j + self.s[self.i] as usize) % 256;
        
        self.s.swap(self.i, self.j);

        Some(self.s[(self.s[self.i] as usize + self.s[self.j] as usize) % 256])
    }
}




