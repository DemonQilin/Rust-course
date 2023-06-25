use blog_shared::Post;

fn main() {
    let post = Post::new(
        "Rust in the Frontend".to_owned(),
        "Rust compile to WASM".to_owned(),
    );

    println!("{post:?}");
}
