extern crate json;
extern crate notify_rust;

fn main()
{
    
    let passed_json_str = std::env::args().nth(1).unwrap_or(String::from("undefined"));

    if &passed_json_str == "undefined" { eprintln! ("failed"); }

    let notif = json::parse(&passed_json_str).unwrap();

    let wait_time = notif["time"].to_string().parse::<u64>().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(wait_time));

    let req_urgency = match &notif["urgency"].as_i8().unwrap_or(1)
    {
        2 => notify_rust::Urgency::Normal,
        3 => notify_rust::Urgency::Critical,
        _ => notify_rust::Urgency::Low
    };

    notify_rust::Notification::new().summary(&notif["title"].to_string()).body(&notif["body"].to_string()).icon(&notif["icon"].to_string()).urgency(req_urgency).show().unwrap();
}
