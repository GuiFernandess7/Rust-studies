extern crate chrono;

use chrono::{NaiveDate};

pub trait PaymentMethod {
    fn pay(&self, amount: f32) -> ();
}

struct CreditCard {
    number: String,
    cvv: i32,
    expiry: NaiveDate
}

struct Paypal {
    email: String
}

impl PaymentMethod for CreditCard {
    fn pay(&self, amount: f32) -> () {
        println!("Payment made successfully!");
    }
}

impl PaymentMethod for Paypal {
    fn pay(&self, amount: f32) -> () {
        println!("Payment made successfully!");
    }
}

fn main() {
    let my_credit_card: CreditCard = CreditCard{
        number: String::from("1232.4561.7890.3211"),
        cvv: 234,
        expiry: NaiveDate::from_ymd(2021, 8, 1)
    };

    let paypal: Paypal = Paypal{
        email: String::from("gfssp@email.com")
    };
}
