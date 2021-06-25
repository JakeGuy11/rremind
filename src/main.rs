extern crate json;
extern crate async_process;
extern crate notify_rust;

use notify_rust::Notification;

// Any globals
const INSTANT_RREMIND_PATH: &str = "./instant_rremind";

// Take a nw-nd-nh-nm-ns and return the seconds
fn countdown_to_seconds(req_time: &String) -> u64
{
    let mut total_secs: u64 = 0;

    for seg in req_time.split("-")
    {
        let mut checkable_seg = seg.to_string();
        let date_id = checkable_seg.pop().unwrap_or('m');
        match date_id
        {
            'w' => { total_secs += checkable_seg.parse::<u64>().unwrap_or(0) * 604800; },
            'd' => { total_secs += checkable_seg.parse::<u64>().unwrap_or(0) * 86400; },
            'h' => { total_secs += checkable_seg.parse::<u64>().unwrap_or(0) * 3600; },
            'm' => { total_secs += checkable_seg.parse::<u64>().unwrap_or(0) * 60; },
            's' => { total_secs += checkable_seg.parse::<u64>().unwrap_or(0); },
            _ => { eprintln! ("DateTime identifier '{}' not recognized, ignoring option!", date_id); }
        }
    }
    total_secs
}

fn instant_notif(notif: json::JsonValue)
{
    async_process::Command::new(INSTANT_RREMIND_PATH).arg(notif.dump()).spawn().unwrap();
}

// The user has selected instant mode
fn queue_instant()
{
    let args: Vec<String> = std::env::args().skip(3).collect();

    let mut notif = json::object!
    {
        title: "rremind",
        body: "You did not set any body text for this reminder",
        icon: "/home/jake/downloads/ogayu.jpg",
        urgency: 1,
        time: -1
    };

    for i in 0..args.len()
    {
        match args[i].as_str()
        {
            "-s" => { notif["title"] = json::JsonValue::String(args[i+1].to_string()); },
            "-b" => { notif["body"] = json::JsonValue::String(args[i+1].to_string()); },
            "-i" => { notif["icon"] = json::JsonValue::String(args[i+1].to_string()); },
            "-u" => { notif["urgency"] = json::JsonValue::Number(args[i+1].parse::<i32>().unwrap_or(1).into()); },
            "-t" => { notif["time"] = json::JsonValue::Number(countdown_to_seconds(&args[i+1]).into()); },
            _ => {  }
        }
    }
    instant_notif(notif);
}

// This function will start the periodic loop that checks for notifications
fn start_loop()
{
    println! ("Once start is implemented, it will be here");
    std::process::exit(0);
}

fn main()
{
    // Check if we want to start in add or start mode
    let intent = std::env::args().nth(1).unwrap_or(String::from("start"));

    // If the user wants to start, call start_loop
    if &intent == "start" { start_loop(); }
    else if &intent != "add" { eprintln! ("You must define a valid intent!"); std::process::exit(1); }

    // Check what add mode they want to use
    let mode = std::env::args().nth(2).unwrap_or(String::from("undefined"));

    // Check what they want to do
    match mode.as_str()
    {
        "s" => { println! ("You've selected single mode"); },
        "i" => { queue_instant(); },
        "r" => { println! ("You've selected reccurant mode"); },
        _ => { println! ("Please enter a valid mode"); }
    }

    // Notification::new().summary("Title here").body("This is the body of the notification").icon("/home/jake/downloads/ogayu.jpg").show().expect("Some sort of error occurred!");
}
