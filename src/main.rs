extern crate chrono;
extern crate json;
extern crate async_process;
extern crate notify_rust;
extern crate home;

use chrono::{Datelike, Timelike, TimeZone};
use std::io::{Read, Write};

// Any globals
const RREMIND_SUFFIX: &str = ".rremind";

// Take a nw-nd-nh-nm-ns and return the seconds
fn countdown_to_time(req_time: &String) -> String
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

    // Now we have the total seconds - add it to the time now and format it properly
    let target_time = chrono::Local::now() + chrono::Duration::seconds(total_secs as i64);

    let formatted_datetime = format! ("{}_{}_{}_{}_{}_{}", target_time.year(), target_time.month(), target_time.day(), target_time.hour(), target_time.minute(), target_time.second());

    formatted_datetime
}

fn queue_single(entry_dir: &mut std::path::PathBuf)
{
    let args: Vec<String> = std::env::args().skip(3).collect();

    let mut notif = json::object!
    {
        title: "rremind",
        body: "You did not set any body text for this reminder",
        icon: "/home/jake/downloads/ogayu.jpg",
        urgency: 1,
        time: "120"
    };

    let mut target_datetime = [chrono::Local::now().year() as u32, chrono::Local::now().month(), chrono::Local::now().day(), chrono::Local::now().hour(), chrono::Local::now().minute(), chrono::Local::now().second()];

    for i in 0..args.len()
    {
        match args[i].as_str()
        {
            "-t" | "--title" => { notif["title"] = json::JsonValue::String(args[i+1].to_string()); },
            "-b" | "--body" => { notif["body"] = json::JsonValue::String(args[i+1].to_string()); },
            "-i" | "--icon" => { notif["icon"] = json::JsonValue::String(args[i+1].to_string()); },
            "-u" | "--urgency" => { notif["urgency"] = json::JsonValue::Number(args[i+1].parse::<i32>().unwrap_or(1).into()); },
            "-y" | "--year" => { target_datetime[0] = args[i+1].parse::<u32>().expect("Failed to parse year!"); },
            "-o" | "--month" => { target_datetime[1] = args[i+1].parse::<u32>().expect("Failed to parse month!"); },
            "-d" | "--day" => { target_datetime[2] = args[i+1].parse::<u32>().expect("Failed to parse day!"); },
            "-h" | "--hour" => { target_datetime[3] = args[i+1].parse::<u32>().expect("Failed to parse hour!"); },
            "-m" | "--minute" => { target_datetime[4] = args[i+1].parse::<u32>().expect("Failed to parse minute!"); },
            "-s" | "--second" => { target_datetime[5] = args[i+1].parse::<u32>().expect("Failed to parse second!"); },
            _ => {  }
        }
    }

    notif["time"] = json::JsonValue::String(format! ("{}_{}_{}_{}_{}_{}", target_datetime[0], target_datetime[1], target_datetime[2], target_datetime[3], target_datetime[4], target_datetime[5]));

    write_notif(notif, entry_dir);

}

fn write_notif(notif: json::JsonValue, entry_dir: &mut std::path::PathBuf)
{
    std::fs::create_dir_all(&entry_dir).unwrap();
    let mut entry_name = notif["title"].to_string() + "." + &chrono::Local::now().timestamp().to_string();
    entry_name.push_str(RREMIND_SUFFIX);
    entry_dir.push(entry_name);
    let mut entry_file = std::fs::File::create(&entry_dir).unwrap();
    entry_file.write_all(notif.dump().as_bytes()).unwrap();    
}

// The user has selected instant mode
fn queue_instant(entry_dir: &mut std::path::PathBuf)
{
    let args: Vec<String> = std::env::args().skip(3).collect();

    let mut notif = json::object!
    {
        title: "rremind",
        body: "You did not set any body text for this reminder",
        icon: "/home/jake/downloads/ogayu.jpg",
        urgency: 1,
        time: "120"
    };

    for i in 0..args.len()
    {
        match args[i].as_str()
        {
            "-t" | "--title" => { notif["title"] = json::JsonValue::String(args[i+1].to_string()); },
            "-b" | "--body" => { notif["body"] = json::JsonValue::String(args[i+1].to_string()); },
            "-i" | "--icon" => { notif["icon"] = json::JsonValue::String(args[i+1].to_string()); },
            "-u" | "--urgency" => { notif["urgency"] = json::JsonValue::Number(args[i+1].parse::<i32>().unwrap_or(1).into()); },
            "-c" | "--countdown" => { notif["time"] = json::JsonValue::String(countdown_to_time(&args[i+1])); },
            // C is for countdown here
            _ => {  }
        }
    }
    
    write_notif(notif, entry_dir);
    queue_start(true, entry_dir);
}

fn queue_start(is_async: bool, entry_dir: &std::path::PathBuf)
{
    let pgrep_out = String::from_utf8(std::process::Command::new("pgrep").arg("rremind").output().expect("failed to pgrep").stdout).unwrap();
    let mut split_pgrep = pgrep_out.split("\n").filter(|i|i != &"").collect::<Vec<&str>>();
    split_pgrep.pop();

    for pid in split_pgrep.into_iter() { std::process::Command::new("kill").arg(pid).spawn().expect("Failed to kill existing instance of rremind!"); }

    if is_async { async_process::Command::new("./rremind").arg("start").spawn().expect("failed to start as daemon!"); std::process::exit(0); }
    else { start_loop(entry_dir); }
}

// This function will start the periodic loop that checks for notifications
fn start_loop(entry_dir: &std::path::PathBuf)
{
    loop
    {
        //First, iterate through all the files in the config dir
        for current_entry_attempt in std::fs::read_dir(entry_dir.as_path()).unwrap()
        {
            let current_entry = match current_entry_attempt
            {
                Ok(..) => { 
                    let unwrapped_item = current_entry_attempt.unwrap();
                    if unwrapped_item.path().is_dir() { continue; }
                    else { unwrapped_item.path() }
                },
                Err(..) => { continue; }
            };
            let mut current_file = std::fs::File::open(current_entry.as_path()).unwrap();
            let mut result_string = String::new();
            current_file.read_to_string(&mut result_string).unwrap();
            
            let notif_read = json::parse(&result_string);
            if let Err(e) = notif_read { eprintln! ("Failed to read contents: {}", e); continue; }
            let notif = notif_read.unwrap();
            
            // Check if it's time to notify
            let time_string = notif["time"].to_string();
            let time_vec = time_string.split("_").collect::<Vec<&str>>();
            
            let wanted_date = chrono::Local.ymd(time_vec[0].parse::<i32>().expect("Failed to parse year"), time_vec[1].parse::<u32>().expect("Failed to parse month"), time_vec[2].parse::<u32>().expect("Failed to parse day")).and_hms(time_vec[3].parse::<u32>().expect("Failed to parse hour"), time_vec[4].parse::<u32>().expect("Failed to parse minute"), time_vec[5].parse::<u32>().expect("Failed to parse seconds"));
            let is_time = wanted_date.timestamp() <= chrono::Local::now().timestamp();
            
            if is_time
            {
                let req_urgency = match &notif["urgency"].as_i8().unwrap_or(1)
                {
                    2 => notify_rust::Urgency::Normal,
                    3 => notify_rust::Urgency::Critical,
                    _ => notify_rust::Urgency::Low
                };
                notify_rust::Notification::new().summary(&notif["title"].to_string()).body(&notif["body"].to_string()).icon(&notif["icon"].to_string()).urgency(req_urgency).show().unwrap();
                std::fs::remove_file(current_entry.as_path()).unwrap();
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(900));
    }
}

fn main()
{
    // Set the entry dir
    let mut entry_dir = home::home_dir().unwrap();
    entry_dir.push(".local");
    entry_dir.push("share");
    entry_dir.push("rremind");

    // Check if we want to start in add or start mode
    let intent = std::env::args().nth(1).unwrap_or(String::from("start"));

    // If the user wants to start, call start_loop
    if &intent == "start" { queue_start(false, &entry_dir); }
    else if &intent != "add" { eprintln! ("You must define a valid intent!"); std::process::exit(1); }

    // Check what add mode they want to use
    let mode = std::env::args().nth(2).unwrap_or(String::from("undefined"));

    // Check what they want to do
    match mode.as_str()
    {
        "s" | "single" => { queue_single(&mut entry_dir); },
        "i" | "instant" => { queue_instant(&mut entry_dir); },
        "r" | "reccuring" => { println! ("You've selected reccurant mode"); },
        _ => { println! ("Please enter a valid mode"); }
    }
}
