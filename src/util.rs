use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;

/// Generate a random string of alphanumeric characters
pub fn random_string(n: usize) -> String {
    iter::repeat(())
        .map(|()| thread_rng().sample(Alphanumeric))
        .take(n)
        .collect()
}
