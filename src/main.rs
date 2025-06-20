mod hello_rust;

fn main() {
    println!("Hello, world!");

    //this is comment
    println!("i'm Rust 0x1jar");

    //run progame hello_rust
    let run: &'static str = hello_rust::hello_rust();

    println!("{:?}",run);
    
}


