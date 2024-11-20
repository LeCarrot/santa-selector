use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn santa_shuffle<T, F>(vec: &Vec<T>, matching:F) -> Vec<T> 
where 
    T: Clone,
    F: Fn(&Vec<T>) -> bool    
{
    let mut new_vec = vec.clone();

    new_vec.shuffle(&mut thread_rng());

    if matching(&new_vec) {
        new_vec = santa_shuffle(&new_vec, matching);
    }

    new_vec.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_thousand_shuffles() {
        let gifters = vec!["Aric", "Joe", "Josh", "Dean", "Aaron"];

        let matching = |x: &Vec<&str>| -> bool {
            gifters.iter()
                .zip(x.iter())
                .filter(|&(a, b)| a == b)
                .count() > 0
        };

        for _i in 0..1000 {
            let giftees = santa_shuffle(&gifters, matching);

            //confirm not matching
            let no_matching_elements = gifters.iter()
                .zip(giftees.iter())
                .filter(|&(a, b)| a == b)
                .count() == 0;
            
            assert!(no_matching_elements);
        }
    }
}
