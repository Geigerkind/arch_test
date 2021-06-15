pub use a::b::c;
pub use file_1::Test1;

mod file_1;

fn main() {
    let a = Test1;
}