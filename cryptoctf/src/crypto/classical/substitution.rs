use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

pub struct Substitution<T: Eq + Hash + Clone + Debug> {
    dic_from: HashMap<T, T>,
    dic_to: HashMap<T, T>,
}

impl<T: Eq + Hash + Clone + Debug> Substitution<T> {

    pub fn new(from: &[T], to: &[T]) -> Option<Self>{
        
        if from.len() != to.len() {
            println!("{} != {}", from.len(), to.len());
            return None;
        }

        let mut sub = Substitution { dic_from: HashMap::new(), dic_to: HashMap::new() };


        for (f, t) in from.iter().zip(to.iter()) {
           sub.dic_from.insert(f.clone(), t.clone());
           sub.dic_to.insert(t.clone(), f.clone());
        }

        Some(sub)
    }

    pub fn encrypt(&self, data: &[T]) -> Option<Vec<T>> {
        let mut result = Vec::new();
        
        for item in data.iter() {
            match self.dic_from.get(item) {
                Some(sub) => result.push(sub.clone()),
                None => return None,
            }
        }

        Some(result)
    }

    pub fn decrypt(&self, data: &[T]) -> Option<Vec<T>> {
        let mut result = Vec::new();

        for item in data.iter() {
            match self.dic_to.get(item) {
                Some(sub) => result.push(sub.clone()),
                None => return None, 
            }
        }

        Some(result)
    }   

}
