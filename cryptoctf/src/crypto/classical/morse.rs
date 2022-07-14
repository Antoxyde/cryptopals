use classical::substitution::Substitution;
use utils::m_split;

pub struct Morse<'a> {
    sub: Substitution<&'a str>,
}

impl<'a> Default for Morse<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Morse<'a> {
    pub fn new() -> Self {
        let from = m_split("ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890", 1);
        let to = vec![".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--..", "-----", ".----", "..---", "...--", "....-", ".....", "-....", "--...", "---..", "----."];

        Morse{ sub: Substitution::new(&from, &to).unwrap() }
    }

    pub fn encode(&self, data: &[&'a str]) -> Option<Vec<&str>> {
        self.sub.encrypt(data)
    }

    pub fn decode(&self, data: &[&'a str]) -> Option<Vec<&str>> {
        self.sub.decrypt(data)
    }

    pub fn encode_str(&self, data: &'a str) -> Option<String> {
         match self.sub.encrypt(&m_split(data, 1)) {
             Some(encoded) => Some(encoded.join(" ")),
             None => None,
        }
          
    }

    pub fn decode_str(&self, data: &'a str) -> Option<String> {
         match self.sub.decrypt(&data.split(' ').collect::<Vec<&str>>()) {
             Some(decoded) => Some(decoded.concat()),
             None => None,
        }
    }

}
