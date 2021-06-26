extern crate chrono;
extern crate json;
extern crate async_process;
extern crate notify_rust;
extern crate home;

//use notify_rust::Notification;
use chrono::{Datelike, Timelike};
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

    let formatted_datetime = format! ("{}_{}_{}-{}_{}_{}", target_time.year(), target_time.month(), target_time.day(), target_time.hour(), target_time.minute(), target_time.second());

    formatted_datetime
}

fn instant_notif(notif: json::JsonValue, entry_dir: &mut std::path::PathBuf)
{
    std::fs::create_dir_all(&entry_dir).unwrap();
    let mut entry_name = std::fs::read_dir(&entry_dir).unwrap().count().to_string();
    entry_name.push_str(RREMIND_SUFFIX);
    entry_dir.push(entry_name);
    let mut entry_file = std::fs::File::create(&entry_dir).unwrap();
    entry_file.write_all(notif.dump().as_bytes()).unwrap();
    async_process::Command::new("./rremind").arg("start").spawn().expect("failed to start as daemon!");
    //async_process::Command::new(INSTANT_RREMIND_PATH).arg(notif.dump()).spawn().unwrap();
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
        time: "nah"
    };

    for i in 0..args.len()
    {
        match args[i].as_str()
        {
            "-s" => { notif["title"] = json::JsonValue::String(args[i+1].to_string()); },
            "-b" => { notif["body"] = json::JsonValue::String(args[i+1].to_string()); },
            "-i" => { notif["icon"] = json::JsonValue::String(args[i+1].to_string()); },
            "-u" => { notif["urgency"] = json::JsonValue::Number(args[i+1].parse::<i32>().unwrap_or(1).into()); },
            "-t" => { notif["time"] = json::JsonValue::String(countdown_to_time(&args[i+1])); },
            _ => {  }
        }
    }
    
    //if notif["time"].as_i64().unwrap() < 10 { eprintln! ("You cannot select a time lower than 10 seconds."); }
    //else { instant_notif(notif, entry_dir); }
    instant_notif(notif, entry_dir);
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
            current_file.read_to_string(&mut result_string);
            
            let notif_read = json::parse(&result_string);
            if let Err(e) = notif_read { eprintln! ("Failed to read contents: {}", e); continue; }
            let notif = notif_read.unwrap();
            println! ("time is {}", notif["time"].to_string());
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
    if &intent == "start" { start_loop(&entry_dir); }
    else if &intent != "add" { eprintln! ("You must define a valid intent!"); std::process::exit(1); }

    // Check what add mode they want to use
    let mode = std::env::args().nth(2).unwrap_or(String::from("undefined"));

    // Check what they want to do
    match mode.as_str()
    {
        "s" => { println! ("You've selected single mode"); },
        "i" => { queue_instant(&mut entry_dir); },
        "r" => { println! ("You've selected reccurant mode"); },
        _ => { println! ("Please enter a valid mode"); }
    }

    // Notification::new().summary("Title here").body("This is the body of the notification").icon("/home/jake/downloads/ogayu.jpg").show().expect("Some sort of error occurred!");
}
