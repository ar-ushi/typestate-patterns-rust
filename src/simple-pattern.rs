#[derive(Debug)]
struct A;

#[derive(Debug)]
struct B;

fn one(_: A) -> B {
    B
}

fn main() {
    let a = A;
    let b = one(a); // transitions to b 
    println!("{:?}", b); // Use Debug trait for formatting
}
