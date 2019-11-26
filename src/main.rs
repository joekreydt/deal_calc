use std::io::{stdin, self, Write};

fn main() {
    let mut s = String::new();
    let mut s2 = String::new();
    let mut country = String::new();
    let mut country_int: u8 = 0;

    while country_int != 1 && country_int != 2 {
        print!("Enter 1 for US or 2 for UK: ");
        io::stdout().flush().unwrap();
        country_int = 0;
        country.clear();
        stdin().read_line(&mut country)
            .expect("Did not enter a correct string");
        match country.trim().parse::<u8>() {
            Ok(_s) => country_int = country.trim().parse().unwrap(),
            Err(_err) => println!("Please enter 1 or 2")
        }
    }

    if country_int == 1 {
        println!("You chose US")
    } else if country_int == 2 {
        println!("You chose UK")
    } else {
        println!("You chose wrong. Using default, US")
    }

    print!("Enter the discount percent: ");
    io::stdout().flush().unwrap();

    loop {
        s.clear();
        stdin().read_line(&mut s)
            .expect("Did not enter a correct string");
        match s.trim().parse::<f64>() {
            Ok(_s) => break,
            Err(_err) => println!("Please enter a number")
        }
    }

    println!("Discount percent: {}", s);

    const ONE_HUNDRED: f64 = 100.;
    let discount_percent: f64 = s.trim().parse().unwrap();

    print!("Enter the original price: ");
    io::stdout().flush().unwrap();

    stdin().read_line(&mut s2)
        .expect("Did not enter a correct string");

    println!("Original price: {}", s2);

    let original_price: f64 = s2.trim().parse().unwrap();

    fn calc_discount(o: f64, d: f64) -> f64 {
        // calculates the price after applying discount
        let dollars_off = o * d / ONE_HUNDRED;
        let price = o - dollars_off;
        return price;
    }

    let price = calc_discount(original_price, discount_percent);
    
    if country_int == 2 {
        println!("Sale price is £{:?}", price)
    } else {
        println!("Sale price is ${:?}", price)
    }

    let discount_percents = [10., 15., 20., 25., 30., 40., 50., 60., 75.];
    println!("");
    for i in discount_percents.iter() {
        print!("If {}% discount,", *i);
        println!("then sale price is ${:?}", calc_discount(original_price, *i))
    }

}