use std::collections::HashMap; 
pub struct ColumnarTransposition {
    pub keyword: Vec<u8>, }

impl ColumnarTransposition {
    
    pub fn new(kw: &[u8]) -> Option<Self> {
        if kw.is_empty() {
            None
        } else {
            Some( ColumnarTransposition { keyword: kw.to_owned()})
        }
    }

    pub fn kw_to_indices(kw: &[u8]) -> Vec<usize> {
        /*
         * We count element's occurences
         * and then get index of element in sorted array
         * and add it's occurence to get it's final index
         */

        let mut h = HashMap::new();  

        for l in kw {
            let entry = h.entry(l).or_insert(0);
            *entry += 1;
        }

        let mut res = Vec::new();
        let mut k = Vec::from(kw);
        k.sort();

        for l in kw {
            let index = k.iter().position(|c| c == l).unwrap();
            let occurence = h.get_mut(l).unwrap();
            res.push(*occurence + index - 1);
            *occurence -= 1;
        }
        let l = res.len();
        let mut result = res.clone();

        for i in 0..l {
            result[res[i]] = i;
        }

        result
       
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {

        let mut columns =  Vec::new();
        let kw = ColumnarTransposition::kw_to_indices(&self.keyword); 

        for i in 0..self.keyword.len() {

            columns.push(Vec::new());  

            for j in (0..data.len()).step_by(self.keyword.len()) {
                if data.len() > i + j {
                    columns[i].push(data[i + j]);
                }
            }
        }
        
        let mut result = Vec::new();

        for indice in kw {
            result.append(&mut columns[indice]);
        }

        result
    }

    pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {

        let t = data.len() % self.keyword.len();
        let kw = ColumnarTransposition::kw_to_indices(&self.keyword);

        let t_highers = if t != 0 {
            Vec::new()
        } else {
            let mut kwtmp = kw.clone();
            kwtmp.sort();
            Vec::from(&kwtmp[t..])
        };

        let mut columns = Vec::new();
        let mut pos = 0;
        let min_col_length = data.len() / kw.len();

        for i in 0..kw.len() {
            if t == 0 || t_highers.contains(&kw[i]) {
                columns.push(&data[pos..pos + min_col_length]);
                pos += data.len() / kw.len();
            } else {
                columns.push(&data[pos..=pos + min_col_length]);
                pos += (data.len() / kw.len()) + 1
            }
        }

        let mut columns_ordered = Vec::new();
        let l = kw.len();

        for i in 0..kw.len() {
            let index =  kw.iter().position(|&p| p == i).unwrap();
            columns_ordered.push(columns[index]);
        }

        let mut result:Vec<u8> = Vec::new();

        for i in 0..data.len() {
            result.push(columns_ordered[i % l][i / l]);
        }

        result
    }
}
