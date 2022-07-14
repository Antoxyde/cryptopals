pub struct PolybiusSquare {
    keyword: Vec<u8>,
    alphabet: Vec<u8>,
}

impl PolybiusSquare {
    
    pub fn new(kw: &[u8], alphabet: &[u8]) -> Result<Self, String> {
        if kw.len() * kw.len() == alphabet.len() {
            Ok(PolybiusSquare {
                        keyword: kw.to_owned(),
                        alphabet: alphabet.to_owned(),
            })
        } else {
            Err(String::from("Alphabet length should be the square of the keyword length"))
        }
    }

    fn cleanup(&self, data: &[u8]) -> Vec<u8> {
        data.iter().filter(|&c| self.alphabet.contains(c)).copied().collect()
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {

        let plaintext = self.cleanup(data);
        let mut result = Vec::new();
        
        for c in plaintext.iter() {
            let index = self.alphabet.iter().position(|p| p == c).unwrap();
            result.push(self.keyword[index / self.keyword.len()]);
            result.push(self.keyword[index % self.keyword.len()]);
        }

        result
    }

    pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {

        let mut result = Vec::new();

        for i in (0..data.len()).step_by(2) {
            if i + 1 < data.len() {
                let first = self.keyword.iter().position(|&p| p == data[i as usize]).unwrap();
                let second = self.keyword.iter().position(|&p| p == data[(i + 1) as usize ]).unwrap();
                result.push(self.alphabet[first * self.keyword.len() + second]);
            }
        }

        result
    }
}
