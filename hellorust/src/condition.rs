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