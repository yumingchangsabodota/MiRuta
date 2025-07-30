
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::cost::Cost;


#[derive(Debug, Deserialize, Serialize)]
pub enum PlanItemType {
    Accomodation,
    Transport,
    Food,
    Site,
    Document,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlanItem {
    pub item_type: PlanItemType,
    pub name: String,
    pub description: String,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    pub cost: Cost,
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use iso_currency::{Currency};

    use crate::cost::Cost;

    use support::data_storage::{save_bin, read_bin};

    use crate::plan_item::{PlanItem, PlanItemType};
    #[test]
    fn show_struct() {
        let plan_item = PlanItem{
            item_type: PlanItemType::Transport,
            name: String::from("test_item"),
            description: String::from("test_item desc"),
            from: Utc::now(),
            to: Utc::now(),
            cost: Cost{cost:-1, currency: Currency::USD},
        };
        print!("{:?} \n", plan_item);

        save_bin("../save_plan_item.bin", plan_item);
        
        let read_data: PlanItem = read_bin("../save_plan_item.bin").expect("Failed to load data");
        print!("Read from ../save_plan_item.bin \n");
        print!("{:?} \n", read_data);

    }
}
