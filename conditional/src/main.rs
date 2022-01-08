fn if_statement()
{
    let temp = 25;
    if temp > 30 {
        println!("Very hot outside");
    }
    else if temp<10{
        println!("very cold outside");
    }
    else{
        println!("Nice weather outside");
    }
    let day= if temp<20 {"cloudy"} else {"sunny"};
    println!{"It is {} today",day};
}

fn for_loop()
{
    for x in 1..10
    {
        println!("X={}",x);
    }
    for (pos,y) in (30..41).enumerate()
    {
        println!("{} :{}",pos,y);
    }

}
fn match_statement(){
    let country_code = 91;
    let country= match country_code
    {
        44 => "UK",
        46 =>"Sweden",
        7 =>"Russia",
        91 =>"India",
        1...999 =>"Unknown",
        _ =>"Invalid"
    };
    println!("Country is {}",country)
}

fn main()
{
    if_statement();
    for_loop();
    match_statement()

}
