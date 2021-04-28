use diesel::prelude::*;
use remolder::db::models::File;
use remolder::db::{self, establish_connection};

fn main() {
    use db::schema::files::dsl::*;

    let connection = establish_connection();
    let results = files
        .limit(5)
        .load::<File>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} files", results.len());
    for file in results {
        println!(
            "# {:<6} {:<32} {:>10} {} {}",
            file.id, file.hash, file.size, file.path, file.mime
        );
    }
}
