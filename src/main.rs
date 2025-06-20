//import module
mod hello_rust;
mod project {
    pub mod gues_progame;
    pub mod generate_rondom;
}

fn main() {
    println!("Hello, world!");

    //this is comment
    //run progame
    println!("i'm Rust 0x1jar");

    //run progame hello_rust
    let run: &'static str = hello_rust::hello_rust();

    println!("{:?}",run);

    //run progame gues_progame
    //let run1: () = 
    project::gues_progame::run_progame();

    // println!("{:?}", run1);

    project::generate_rondom::run_rondom();

}


