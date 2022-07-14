use classical::columnar_transposition::ColumnarTransposition;
use classical::polybius_square::PolybiusSquare;

pub struct Adfgvx {
    pub ps: PolybiusSquare,
    pub ct: ColumnarTransposition,
    pub _kw_len: usize,
}

impl Adfgvx {

    pub fn new(alphabet: &[u8], keyword: &[u8]) -> Option<Self> {

       if alphabet.len() == 36 {
           Some( Adfgvx {
               ps: PolybiusSquare::new(b"ADFGVX", alphabet).unwrap(),
               ct: ColumnarTransposition::new(keyword).unwrap(),
               _kw_len: keyword.len(),
            })
       } else {
           None
       }
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {

        let subed = self.ps.encrypt(&data);

        self.ct.encrypt(&subed)
    }

    pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        self.ps.decrypt(&self.ct.decrypt(data))
    }

}
