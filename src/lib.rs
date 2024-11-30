use rand::thread_rng;
use rand::seq::SliceRandom;


/*
*   goal: 
*   let pairs: Vec<(&str, &str)> = gifters.santa_shuffle();
*
*   Flow:
*   1. start with v: Vec<T> - &self gives me &Vec<T>. Is this a problem?
*   2. get v_ref: Vec<&T> where each element is a 1:1 reference of v
*   3. get v_ref_clone: Vec<&T> where each element is a 1:1 reference of v
*   4. shuffle v_ref_clone
*   5. zip v_ref and v_ref_clone into pairs_ref: (&&T, &&T)
*   6. map over pairs_ref and dereferance a layer so we have pairs: (&T, &T)
*
*   1. we actually have &Vec<&T>
*   2. v = *self.clone()?
*   3. 
*/

pub trait SantaShuffle<T> {
    fn santa_shuffle(&self) -> Vec<(&T, &T)>; 
}

impl<T: std::cmp::PartialEq> SantaShuffle<T> for Vec<T>
{
    fn santa_shuffle(&self) -> Vec<(&T, &T)> {
        let vec: Vec<&T> = self.iter().collect(); // convert &Vec<T> -> Vec<&T>
        let mut new_vec = vec.clone();
        new_vec.shuffle(&mut thread_rng());

        let mut pairs: Vec<(&T, &T)> = 
            vec
            .iter()
            .zip(new_vec.iter())
            .map( |x| (*x.0, *x.1) )
            .collect();

        if pairs.iter().any(|x| x.0 == x.1) {
            pairs = self.santa_shuffle();
        }

        pairs
    }
}




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
    fn test_trait() {
        let gifters = vec![
            String::from("Greg"), 
            String::from("Joe"), 
            String::from("Bob"), 
            String::from("Jeff"), 
            String::from("Aaron")
        ];

        let pairs = gifters.santa_shuffle();
        assert!(pairs.iter().all( |&x| x.0 != x.1 ));
    }
    
    #[test]
    fn test_trait_with_str() {
        let gifters = vec![
            "Greg", 
            "Joe", 
            "Bob", 
            "Jeff", 
            "Aaron"
        ];

        let pairs = gifters.santa_shuffle();
        assert!(pairs.iter().all( |&x| x.0 != x.1 ));
    }

    #[test]
    fn one_thousand_shuffles() {
        let gifters = vec!["Jim", "Joe", "Bob", "Jeff", "Aaron"];

        let matching = |x: &Vec<&str>| -> bool {
            gifters.iter()
                .zip(x.iter())
                .filter(|&(a, b)| a == b)
                .count() > 0
        };

        for _i in 0..1000 {
            let giftees = santa_shuffle(&gifters, matching);

            let no_matching_elements = gifters.iter()
                .zip(giftees.iter())
                .filter(|&(a, b)| a == b)
                .count() == 0;
            
            assert!(no_matching_elements);
        }
    }
}
