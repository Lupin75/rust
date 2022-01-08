fn print_value(x:i64){
    println!("value= {}",x);
}
fn increase(x: &mut i32)
{
    *x+=1;
}
fn product(x:i32, y:i32) -> i32
{
    x*y
}
fn function()
{
    print_value(34);
    let mut x=4;
    increase(&mut x);
    println!("Value incremented {}",x);

    let a=3;
    let b=5;
    let p=product(a,b);
    println!("product of {} and {} is {}",a,b,p);
}

struct Point
{
    x: f64,
    y: f64
}
struct Line
{
    start: Point,
    end: Point
}
impl Line{
    fn len(&self) -> f64{
        let dx=self.start.x-self.end.x;
        let dy=self.start.y-self.end.y;
        (dx*dx+dy*dy).sqrt()
    }
}
fn method(){
    let p =Point{x:3.0,y:4.0};
    let q= Point{x:0.0,y:0.0};
    let my_line=Line{start:p,end:q};
    println!("length={ }",my_line.len());
}
fn say_hello(){println!("Hello")}

fn closures(){
    let sh=say_hello;
    sh();
    let plus_one=|x:i32|-> i32{x+1};
    let a=6;
    println!(" {} + 1 = {}",a,plus_one(a));
    let mut two=2;
    {
        let plus_two = |y| {
            let mut z = y;
            z += two;
            z
        };
        println!(" {} + 2 = {}", a, plus_two(a));
    }
    let borrow_two= &mut two;
    //T: by value
    //T&
    //&mut &
    let plus_three=|mut x:  i32|x+=3;
    let mut f=12;
    plus_three( f);
    println!("f={}",f)

}

fn main() {
    //function();
    //method();
    closures();
}
