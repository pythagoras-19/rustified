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
        Dollar::UlyssesSGrant => 50,
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