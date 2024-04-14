pub(crate) enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

pub(crate) enum Dollar {
    GeorgeWashington,
    ThomasJefferson,
    AbrahamLincoln,
    AlexanderHamilton,
    AndrewJackson,
    UlyssesSGrant,
    BenjaminFranklin
}

// for handling failures
enum Result<T, E> {
    // T and E are generics
    Ok(T), // success
    Err(E), // fail
}

pub fn value_in_cents(coin: Coin) -> u8 {
    match  coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}

pub fn value_in_dollars(dollar: Dollar) -> i16 {
    match dollar {
        Dollar::GeorgeWashington => 1,
        Dollar::ThomasJefferson=> 2,
        Dollar::AbrahamLincoln => 5,
        Dollar::AlexanderHamilton => 10,
        Dollar::AndrewJackson => 20,
        Dollar::UlyssesSGrant =>  50,
        Dollar::BenjaminFranklin => {
            im_rich();
            100
        }
    }
}

fn im_rich() {
    for _ in 0..=100 {
        println!("IM RICH!");
    }
}

pub fn convert_to_dollars(amount_in_cents: u32, coin: Coin) -> std::result::Result<f32, &'static str> {
    match coin {
        Coin::Penny => std::result::Result::Ok(amount_in_cents as f32 * 0.01),
        _ => std::result::Result::Err("Conversion is only possible with Penny"),
    }
}