use rand::thread_rng;
use rand::seq::SliceRandom;

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
            .map( |x| (*x.0, *x.1) ) // convert (&&T, &&T) -> (&T, &T)
            .collect();

        if pairs.iter().any(|x| x.0 == x.1) {
            pairs = self.santa_shuffle();
        }

        pairs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
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
    fn test_str() {
        let gifters = vec!["Greg", "Joe", "Bob", "Jeff", "Aaron"];
        let pairs = gifters.santa_shuffle();
        assert!(pairs.iter().all( |&x| x.0 != x.1 ));
    }
}
