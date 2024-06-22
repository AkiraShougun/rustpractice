mod fs;
fn main() {
    println!("Hello there.");
    fs::test_create_dir();
    fs::create_file();
}

fn func() {
    fs::create_file();
}
