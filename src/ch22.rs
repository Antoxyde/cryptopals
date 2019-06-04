/*
Cryptoptals challenge 22 solution
https://www.cryptopals.com/sets/3/challenges/22
*/

#[cfg(test)]
mod test {

    use cryptoctf::rng::mt19937::MT19937;
    use std::{thread, time};
    use rand::{thread_rng, Rng};


    fn get_rnd() -> (u32, u32) {

        let mut rng = thread_rng();

        let mut sleeptime = time::Duration::from_millis(rng.gen::<u64>() % 50000);

        thread::sleep(sleeptime);

        let seed = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH).expect("SystemTime before UNIX EPOCH!").as_secs() as u32; 
        
        let mut mt = MT19937::new(seed);
        let rnd = mt.rnd();

        sleeptime = time::Duration::from_millis(rng.gen::<u64>() % 50000);
        thread::sleep(sleeptime);
        
        return (rnd,seed);   
    }

    fn crack_seed(rnd: u32) -> Option<u32> {
        let t = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH).expect("SystemTime before UNIX EPOCH!").as_secs() as u32;

        for seed in t-1000..t {
            let mut mt = MT19937::new(seed);

            if mt.rnd() == rnd {
                return Some(seed);
            }
        }

        return None

    }

    #[test]
    fn set03_ch22() {
        
        let (rnd, seed) = get_rnd();
        let cracked_seed = crack_seed(rnd).expect("Failed to crack seed.");
        assert_eq!(seed, cracked_seed)
    }
}
