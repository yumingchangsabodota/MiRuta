use dioxus::prelude::*;
use support::data_storage::{read_bin, save_bin};
use travel_plan::plans::{TravelPlans, TravelPlan};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

const STORAGE_FILE_NAME: &str = "travel_plans.bin";

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        MyPlans {}
        LoadPlans {}

    }
}

#[component]
pub fn MyPlans() -> Element {
    rsx! {
        div {
            id: "travel_plans",
            img { src: HEADER_SVG, id: "header" }
            div { id: "plans",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}

#[component]
pub fn LoadPlans() -> Element {
    let mut plans: TravelPlans = read_bin(STORAGE_FILE_NAME).expect("Failed to load plans");
    let mock = vec![0,1,2,3,4,5];


    rsx!{
       for (i, plan) in plans.plans.iter().enumerate(){
            div {"{plan.name}"}
       }
    }
}