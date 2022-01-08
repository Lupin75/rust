#![allow(dead_code)]

struct point{
    x: f64,
    y: f64
}

fn origin()-> point
{
    point{x: 0.0, y: 0.0}
}

pub fn stacknheap()
{
    let p1=origin();
    let p2= Box::new(origin());
}