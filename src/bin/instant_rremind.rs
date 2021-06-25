extern crate json;
extern crate notify_rust;

fn main()
{
    
    let passed_json_str = std::env::args().nth(1).unwrap_or(String::from("undefined"));

    if &passed_json_str == "undefined" { eprintln! ("failed"); }

    println! ("passed string is {}", passed_json_str);

    let notif = json::parse(&passed_json_str).unwrap();

    let wait_time = notif["time"].to_string().parse::<u64>().unwrap();

    println! ("wait time is {}", wait_time);

    std::thread::sleep(std::time::Duration::from_secs(wait_time));

    notify_rust::Notification::new().summary("notif!").show().unwrap();
}
