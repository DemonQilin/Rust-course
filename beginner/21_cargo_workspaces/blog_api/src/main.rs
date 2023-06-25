use blog_shared::Post;

fn main() {
    let post = Post::new("How speak Rust?".to_owned(), "Rust is a chimba".to_owned());

    println!("{post:?}");
}
