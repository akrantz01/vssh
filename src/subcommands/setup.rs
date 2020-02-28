pub fn interactive() {
    println!("Running interactively");
}

pub fn noninteractive<'a>(server: &'a str, tls: bool, token: &'a str, key: &'a str, path: &'a str) {
    println!("Running non-interactively");
    println!("\tServer: {}", server);
    println!("\tTLS: {}", tls);
    println!("\tToken: {}", token);
    println!("\tDefault Key: {}", key);
    println!("\tPath: {}", path);
}
