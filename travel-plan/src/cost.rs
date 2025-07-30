
use serde::{Deserialize, Serialize};
use iso_currency::{Currency};

#[derive(Debug, Deserialize, Serialize)]
pub struct Cost{
    pub cost: i32,
    pub currency: Currency,
}

impl Cost {

    pub fn set_currency(&mut self, currency: Currency) {
        self.currency = currency
    }

    pub fn set_cost(&mut self, cost: i32) {
        self.cost = cost;
    }
}

#[cfg(test)]
mod tests {
    use crate::cost::Cost;
    use iso_currency::{Currency};

    #[test]
    fn test_cost(){
        let mut cost = Cost{ cost: 5, currency:Currency::USD };
        print!("{:?} \n", cost);
        // set currency to AUD
        cost.set_currency(Currency::AUD);
        print!("{:?} \n", cost);
    }
}
