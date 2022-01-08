use std::mem;
fn main() {
    let mut a : u8 = 123;
    println!("a={}",a);
    a = 45;
    println!("a={}",a);
    let mut c = 12345678;
    println!("c={}, size {} bytes",c, mem::size_of_val(&c));
    c=-1;
    println!("c={}, size {} bytes",c, mem::size_of_val(&c));
    let z= 2.6;
    println!("z={}, size {} bytes",z, mem::size_of_val(&z));
    let g=false;
    println!("g={}, size {} bytes",g, mem::size_of_val(&g));
    let f=4>3;
    println!("f={}, size {} bytes",f, mem::size_of_val(&f));

}
