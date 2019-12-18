extern crate rustydxf;

fn main() {
    let a = rustydxf::parse(std::path::Path::new("Hi2.dxf"));
    println!("{:#?}", a);
}