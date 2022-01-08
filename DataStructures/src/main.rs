#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;
use std::collections::HashMap;
//Structures
struct Point
{
    x : f64,
    y: f64,
}

fn structure(){
    let p = Point{
        x: 0.0,
        y: 0.0
    };
    println!("The origin is {} , {}",p.x,p.y);
    let q = Point{
        x: 3.0,
        y: 4.0
    };
    println!("The point is {} , {}",q.x,q.y);
    let dis = (q.x-p.x)*(q.x-p.x)+(q.y-p.y)*(q.y-p.y);
    println!("Square Distance between two points is {}",dis);

}

//Enumerations
enum Color{
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8),
    Cmyk{cyan:u8,magenta:u8,yellow:u8,black:u8}
}
fn enums(){
    let c:Color = Color::Cmyk {cyan:0,magenta:100,yellow:5,black:254};
    match c{
        Color::Blue =>println!("B"),
        Color::Red =>println!("R"),
        Color::Green =>println!("G"),
        Color::RgbColor(0,0,0) |
        Color::Cmyk {cyan:_,magenta:_,yellow:_,black:255} =>println!("Black"),
        Color::RgbColor(r,g,b) =>println!("rgb {},{},{}",r,g,b),
        _ =>println!("Other color")
    }
}

//Unions
union  IntOrFloat
{
    i:i32,
    f: f32
}
fn process_num(iof:IntOrFloat)
{
    unsafe{
        match iof{
            IntOrFloat{i:42} =>println!("Integer is 42"),
            IntOrFloat{f} =>println!("Value is {}",f)
        }
    }
}
fn uni(){
    let mut iof=IntOrFloat{i:7};
    let value = unsafe {iof.i};
    println!("{}",value);
    process_num(iof)

}

//Option T
fn option_t(){
    let x=5.0;
    let y =2.0;
    let result= if y!=0.0 {Some(x/y)}else {None};
    match result{
        Some(z) => println!("{}/{}={}",x,y,z),
        None => println!("Division by zero not possible")
    }
    if let Some(z)= result{
        println!("result = {}",z)
    }
}

//Arrays
fn arrays(){
    let mut a = [1,2,3,4,5];
    println!(" array has {} elements and first element is {} ",a.len(),a[0]);
    a[0]=0;
    println!("first element {}",a[0]);

    println!("{:?}",a);

    if a!=[1,2,3,4,5]{
        print!("Not first 5 natural numbers")
    }

    let b:[u16;10] =[1;10];
    for i in 0..b.len() {
        println!("{}", b[i])
    }
    println!("Array b took {} bytes",mem::size_of_val(&b));

    let mtx:[[i32;3];3]=
    [
        [1,0,0],
        [0,2,0],
        [0,0,3]
    ];
    println!("{:?}",mtx);
    for i in 0..mtx.len()
    {
        for j in 0..mtx[i].len(){
            if i==j
            {
                println!("mtx {},{} = {}",i,j,mtx[i][j])
            }
        }
    }
}

//Vectors
fn vectors()
{
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("A={:?}",a);
    a.push(4);
    println!("A={:?}",a);
    println!("a[0]={}",a[0]);
    let idx:usize = 0;
    a[idx]=1;
    println!("a[0]={}",a[idx]);

    match a.get(2){
        Some(x)=> println!("a[{}]={}",2,a[2]),
        None=> println!("Error no value found")
    }
    for x in &a {println!("{}",x);}
     a.push(5);
    println!("A={:?}",a);
    let la= a.pop();
    println!("Last element was {:?} A{:?}",la,a);
}
//Tuples

fn sum_and_product(x:i32 , y:i32) -> (i32, i32)//Return type
{
    (x+y,x*y)
}
fn tuples()
{
    let x= 15;
    let y= 2;
    let sp=sum_and_product(x,y);
    println!("{:?}",sp);
    println!("{0}+{1}={2} & {0}*{1}={3}",x,y,sp.0,sp.1);
    let (a,b)=sp;
    println!("a={} & b={}",a,b);
    let sp1=sum_and_product(5,1);
    let combined=(sp,sp1);
    println!("{:?}",combined);
    println!("3rd element is {}",(combined.1).0);
    let ((p,q),(r,s))=combined;
    println!("p={},q={},r={},s={}",p,q,r,s);
    let stat = ("true",'a',9,-3.03);
    println!("{:?}",stat)
}

//Stack&heap
struct point
{
    x: f64,
    y: f64
}
fn origin()->point//returns point type
{
    point{x:0.0,y:0.0}
}
pub fn stack_and_heap(){
    let p1 = origin();
    //basically creates a pointer to a boxed memory
    let p2 = Box::new(origin());
    println!("P1 takes up {} bytes", mem::size_of_val(&p1));
    println!("P2 takes up {} bytes", mem::size_of_val(&p2));
    let p3=*p2;
    println!("{},{}",p3.x,p3.y)
}

//Slice
fn use_slice(slice: &mut[i32]){
    println!("first element = {} and length = {} ",slice[0],slice.len());
    println!("{:?}",slice);
    slice[0]=43;
}
fn slices()
{
    let mut data = [1,2,3,4,5];
    use_slice(&mut data[1..4]);
    println!("{:?}",data);
}

//String
fn string()
{
    let s:&'static str="LET LIVE";
    //error: (cannot index) let h =s[0];
    println!("{}",s);
    for c in s.chars()
    {
        println!("{}",c);
    }
    if let Some(first_char) = s.chars().nth(0)
    {
        println!("First char is {}",first_char);
    }
    let mut letters= String::new();
    let mut a='a' as u8;
    while a<='z' as u8
    {
        letters.push(a as char);
        letters.push(',');
        a+=1;
    }
    println!("{}",letters);
    //let u:&str= &letters;
    //let z = letters + "ABC";
    let mut abc = "Hello world".to_string();
    abc.remove(0);
    abc.push_str(
        "!!!"
    );
    println!("{}",abc.replace("ello" , "Hola"))
}

//Hashmap
fn hashmap()
{
    let mut shape=HashMap::new();
    shape.insert(String::from("triangle"),3);
    shape.insert(String::from("square"),4);

    println!("A square has {} sides",shape["square".into()]);

    for (key,value) in &shape{
        println!("{}:{}",key,value);
    }

    shape.insert("square".into(),5);
    println!("{:?}",shape);

    shape.entry("circle".into()).or_insert(0);
    {
        let actual=shape.entry("circle".into()).or_insert(2);
       *actual=5;
    }

    println!("{:?}",shape);
}

//Pattern matching
fn how_many(x:i32) -> &'static str
{
    match x
    {
        0 => "No",
        1|2 => "one or two",
        12 => " a dozen",
        9...11=> " lots of",
        _ if x%2==0 => "even number of",
        _ => "a few"
    }
}
fn pattern_matching(){
    for x in 0..13
    {
        println!("{}:I have {} oranges",x ,how_many(x));
    }

    let point=(0,5);
    match point
    {
        (0,0)=>println!("origin"),
        (0,y)=>println!("Y axis , y = {}",y),
        (x,0)=>println!("X axis , x = {}",x),
        (_,y)=>println!("x=?,y={}",y)

    }
    let c:Color = Color::Cmyk {cyan:0,magenta:100,yellow:5,black:255};
    match c{
        Color::Blue =>println!("B"),
        Color::Red =>println!("R"),
        Color::Green =>println!("G"),
        Color::RgbColor(0,0,0) |
        Color::Cmyk {black:255,..} =>println!("Black"),
        Color::RgbColor(r,g,b) =>println!("rgb {},{},{}",r,g,b),
        _ =>println!("Other color")
    }
}

//Generics
struct Point1<T,V>
{
    x:T,
    y:V
}
fn generics()
{
    let a=Point1{x:0,y:1.23};
    let b=Point1{x:-1.01,y:935.1};
    println!("{},{}",a.x,a.y);
    println!("{},{}",b.x,b.y);
}
fn main() {
    //structure();
    //enums();
    //uni()
    //option_t();
    //arrays();
    //vectors();
    //tuples();
    //stack_and_heap();
    //slices();
    //string();
    //hashmap();
    //pattern_matching();
    generics()
}
