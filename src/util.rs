use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;
use std::process::exit;

/// Generate a random string of alphanumeric characters
pub fn random_string(n: usize) -> String {
    iter::repeat(())
        .map(|()| thread_rng().sample(Alphanumeric))
        .take(n)
        .collect()
}

/// Print an error and exit with error code
pub fn fail(prompt: &'_ str) -> ! {
    leg::error(prompt, None, None);
    exit(1)
}
