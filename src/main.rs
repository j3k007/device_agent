mod models;
mod collector;
use collector::collect_all_info;

fn main() {
    println!("Collecting system info.\n");
    let info = collect_all_info();
    let json = serde_json::to_string_pretty(&info).unwrap();
    println!("{}", json);
}
