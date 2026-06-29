mod vector;
mod vector_iter;

use crate::vector::Vector;

fn main() {
    unsafe {
        let mut v = Vector::new();
        v.push(10);
        v.push(20);
        v.push(30);
        v.push(40);
        v.push(50);

        v.insert(2, 5);

        for i in 0..v.len() {
            println!("{}", v[i]);
        }
        println!("");

        v.pop();
        v.pop();

        for i in 0..v.len() {
            println!("{}", v[i]);
        }
        println!("");

        v.erase(0);

        for i in 0..v.len() {
            println!("{}", v[i]);
        }
        println!("");

        v.reverse();

        for i in 0..v.len() {
            println!("{}", v[i]);
        }
        println!("");

        v.resize(2);

        for i in 0..v.len() {
            println!("{}", v[i]);
        }
        println!("");
    }
}
