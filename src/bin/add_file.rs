use remolder::db;
use std::io::stdin;

fn main() {
    let connection = db::establish_connection();

    println!("Input the hash?");
    let mut hash = String::new();
    stdin().read_line(&mut hash).unwrap();
    let hash = &hash[..(hash.len() - 1)]; // Drop the newline character

    println!("Input the path?");
    let mut path = String::new();
    stdin().read_line(&mut path).unwrap();
    let path = &path[..(path.len() - 1)]; // Drop the newline character

    let r = db::files::add_file(&connection, hash, path, 12345);

    println!("\nSaved draft {} = {:?}", path, r);
}
