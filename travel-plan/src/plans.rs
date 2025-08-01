
use serde::{Deserialize, Serialize};
use support::data_storage::{save_bin, read_bin};

use crate::plan_item::PlanItem;

#[derive(Debug, Deserialize, Serialize)]
pub struct TravelPlan {
    pub plan: Vec<PlanItem>,
}

impl TravelPlan {

    pub fn add_item(&mut self, item: PlanItem){
        self.plan.push(item);
    }

    pub fn remove_item(&mut self, plan_index: usize){
        self.plan.remove(plan_index);
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TravelPlans {
    pub plans: Vec<TravelPlan>
}

impl TravelPlans {

    pub fn new() -> Self {
        Self { 
            plans: Vec::new(), 
        }
    }
    pub fn add_plan(&mut self, plan: TravelPlan){
        self.plans.push(plan);
    }

    pub fn remove_plan(&mut self, plan_index: usize){
        self.plans.remove(plan_index);
    }
}



#[cfg(test)]
mod tests {
    use iso_currency::{Currency};
    use chrono::{NaiveDateTime, TimeZone, Utc};

    use crate::plan_item::{PlanItem, PlanItemType};
    use crate::cost::Cost;

    use crate::plans::{TravelPlan, TravelPlans};
    use support::data_storage::{save_bin, read_bin};
    

    #[test]
    fn add_item(){
        let from_time = NaiveDateTime::parse_from_str("2025-07-30 15:00:00", "%Y-%m-%d %H:%M:%S")
                                                    .expect("Error on Parsing datetime");
        let to_time = NaiveDateTime::parse_from_str("2025-08-05 11:00:00", "%Y-%m-%d %H:%M:%S")
                                                    .expect("Error on Parsing datetime");
        let item_1 = PlanItem{
            item_type: PlanItemType::Accomodation,
            name: String::from("Hyatt"),
            description: String::from("Check in after 15:00"),
            from: Utc.from_utc_datetime(&from_time),
            to: Utc.from_utc_datetime(&to_time),
            cost: Cost{cost:1000, currency: Currency::USD},
        };
        let mut plan = TravelPlan {plan: Vec::new(),};

        plan.add_item(item_1);

        print!("{:?} \n", plan);

        let from_time = NaiveDateTime::parse_from_str("2025-07-29 23:00:00", "%Y-%m-%d %H:%M:%S")
                                                    .expect("Error on Parsing datetime");
        let to_time = NaiveDateTime::parse_from_str("2025-08-05 18:00:00", "%Y-%m-%d %H:%M:%S")
                                                    .expect("Error on Parsing datetime");
        let item_2 = PlanItem{
            item_type: PlanItemType::Transport,
            name: String::from("China Airline"),
            description: String::from("On 2025-07-30 07:00 from TPE to DPS"),
            from: Utc.from_utc_datetime(&from_time),
            to: Utc.from_utc_datetime(&to_time),
            cost: Cost{cost:1000, currency: Currency::USD},
        };
        plan.add_item(item_2);

        print!("{:?} \n", plan);
        print!("remove first");
        plan.remove_item(0);
        print!("{:?} \n", plan);

        let mut plans: TravelPlans = TravelPlans::new();

        plans.add_plan(plan);
        print!("{:?} \n", plans);

        save_bin("../test_data/save_plans.bin", plans);

        let plans: TravelPlans = read_bin("../test_data/save_plans.bin").expect("Failed to load data");
        print!("Read from save_plans.bin \n");
        print!("{:?} \n", plans);


    }
}