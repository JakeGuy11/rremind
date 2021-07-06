extern crate chrono;
extern crate json;
extern crate async_process;
extern crate notify_rust;
extern crate home;

use chrono::{Datelike, Timelike, TimeZone};
use std::io::{Read, Write};

// Any globals go here
const RREMIND_SUFFIX: &str = ".rremind";

// Take a nw-nd-nh-nm-ns and return the seconds
fn _countdown_to_time(req_time: &String) -> String
{
    let mut total_secs: u64 = 0;

    // Split the passed string at '-', each part will be parsed for [value][length_identifier]
    for seg in req_time.split("-")
    {
        let mut checkable_seg = seg.to_string();
        let date_id = checkable_seg.pop().unwrap_or('m');
        match date_id
        {
            // We're only allowing options that have a constant value, ie no months (30 days vs 31), no years (leap years), etc
            'w' => { total_secs += checkable_seg.parse::<u64>().unwrap_or(0) * 604800; }, // weeks
            'd' => { total_secs += checkable_seg.parse::<u64>().unwrap_or(0) * 86400; }, // days
            'h' => { total_secs += checkable_seg.parse::<u64>().unwrap_or(0) * 3600; }, // hours
            'm' => { total_secs += checkable_seg.parse::<u64>().unwrap_or(0) * 60; }, // minutes
            's' => { total_secs += checkable_seg.parse::<u64>().unwrap_or(0); }, // seconds
            _ => { eprintln! ("DateTime identifier '{}' not recognized, ignoring option!", date_id); } // not recognized, skip segment
        }
    }

    // Now we have the total seconds - add it to the time now and format it properly
    let target_time = chrono::Local::now() + chrono::Duration::seconds(total_secs as i64);

    // Format and return the datetime in a way that we can understand later
    let formatted_datetime = format! ("{}_{}_{}_{}_{}_{}", target_time.year(), target_time.month(), target_time.day(), target_time.hour(), target_time.minute(), target_time.second());
    formatted_datetime
}

// Parse the user's options and add a recurring entry
fn queue_recurring(entry_dir: &mut std::path::PathBuf)
{
    // First, add a directory for recurring entries
    entry_dir.push("recurring");

    // Define which recurrance mode to use
    let rec_mode = match std::env::args().nth(3).unwrap().as_str()
    {
        "d" | "daily" => { 1 },
        "w" | "weekly" => { 2 },
        "m" | "monthly" => { 3 },
        "wd" | "weekdays" => { 4 },
        _ => { panic! ("You must set a valid recurrance mode!"); }
    };

    // Define a default notification
    let mut notif = json::object!
    {
        title: "rremind",
        body: "You did not set any body text for this reminder",
        icon: "dialog-information",
        urgency: 1,
        rec_mode: 2, // Default recurrance mode is weekly
        // Here's a little explanation as to how the reccurance will work:
        // The HMS will always be filled out
        // If it's a daily (or weekday) reminder, just the time is needed
        // If it's weekly, a weekday must also be provided and any day_of_month will be ignored
        // If it's monthly, a day_of_month must be provided and any weekday will be ignored
        hour: 12,
        min: 0, // Default time is noon
        sec: 0,
        weekday: "mon", // Default date is monday
        day_of_month: 1, // Default is the first of every month
    };

    // Get the rest of the cli args
    let args: Vec<String> = std::env::args().skip(4).collect();

    // Go through all the args
    for i in 0..args.len()
    {
        match args[i].as_str()
        {
            "-t" | "--title" => { notif["title"] = json::JsonValue::String(args[i+1].to_string()); },
            "-b" | "--body" => { notif["body"] = json::JsonValue::String(args[i+1].to_string()); },
            "-i" | "--icon" => { notif["icon"] = json::JsonValue::String(args[i+1].to_string()); },
            "-u" | "--urgency" => { notif["urgency"] = json::JsonValue::Number(args[i+1].parse::<u32>().expect("Failed to parse urgency!").into()); },
            "-h" | "--hour" => { notif["hour"] = json::JsonValue::Number(args[i+1].parse::<u32>().expect("Failed to parse hour!").into()); },
            "-m" | "--minute" => { notif["min"] = json::JsonValue::Number(args[i+1].parse::<u32>().expect("Failed to parse minute!").into()); },
            "-s" | "--second" => { notif["sec"] = json::JsonValue::Number(args[i+1].parse::<u32>().expect("Failed to parse second!").into()); },
            "-w" | "--weekday" => { notif["weekday"] = json::JsonValue::String(args[i+1].to_string()); },
            "-d" | "--day" => { notif["day_of_month"] = json::JsonValue::Number(args[i+1].parse::<u32>().expect("Failed to parse day of the month!").into()); },
            _ => {  }
        }
    }

    // Assign the recurrance mode
    notif["rec_mode"] = json::JsonValue::Number(rec_mode.into());

    write_notif(notif, entry_dir);

}

// Parse the user's options and add a single entry
fn queue_single(entry_dir: &mut std::path::PathBuf)
{
    // Get the cli args, skipping the first few args that'll always be the same
    let args: Vec<String> = std::env::args().skip(3).collect();

    // Create a default notification object
    let mut notif = json::object!
    {
        title: "rremind",
        body: "You did not set any body text for this reminder",
        icon: "dialog-information",
        urgency: 1,
        time: "120" // the default time will be 120 seconds
    };

    // An array of u32s that represent yyyy, mm, dd, hh, mm, ss for when to send the notification
    let mut target_datetime = [chrono::Local::now().year() as u32, chrono::Local::now().month(), chrono::Local::now().day(), chrono::Local::now().hour(), chrono::Local::now().minute(), chrono::Local::now().second()];

    // Parse all the user's arguments
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

    // Format the datetime to send the notification
    notif["time"] = json::JsonValue::String(format! ("{}_{}_{}_{}_{}_{}", target_datetime[0], target_datetime[1], target_datetime[2], target_datetime[3], target_datetime[4], target_datetime[5]));

    // Write the actual notification entry
    write_notif(notif, entry_dir);
    println! ("Added notification to queue. Make sure you start the daemon!");
}

// Take an object and write it to the entry_dir
fn write_notif(notif: json::JsonValue, entry_dir: &mut std::path::PathBuf)
{
    // Create the directory recursively
    std::fs::create_dir_all(&entry_dir).expect("Failed to create entry directory! Do you have permission?");

    // Create the unique name of the entry, which will be [title].[time since unix epoch].rremind
    let mut entry_name = notif["title"].to_string() + "." + &chrono::Local::now().timestamp().to_string();
    entry_name.push_str(RREMIND_SUFFIX);

    // Add the file to save the entry to to the path
    entry_dir.push(entry_name);

    // Create the file and write all the info to it
    let mut entry_file = std::fs::File::create(&entry_dir).unwrap();
    entry_file.write_all(notif.dump().as_bytes()).unwrap();    
}

// Add an instant notification to the queue
fn queue_instant(entry_dir: &mut std::path::PathBuf)
{
    // Get the cli args, skipping the first few args that'll always be the same
    let args: Vec<String> = std::env::args().skip(3).collect();

    // Create a default notification object
    let mut notif = json::object!
    {
        title: "rremind",
        body: "You did not set any body text for this reminder",
        icon: "dialog-information",
        urgency: 1,
        time: "120" // default time is 2 minutes out
    };

    let mut target_seconds: u64 = 0;

    // Parse all the user's arguments
    for i in 0..args.len()
    {
        match args[i].as_str()
        {
            "-t" | "--title" => { notif["title"] = json::JsonValue::String(args[i+1].to_string()); },
            "-b" | "--body" => { notif["body"] = json::JsonValue::String(args[i+1].to_string()); },
            "-i" | "--icon" => { notif["icon"] = json::JsonValue::String(args[i+1].to_string()); },
            "-u" | "--urgency" => { notif["urgency"] = json::JsonValue::Number(args[i+1].parse::<i32>().unwrap_or(1).into()); },
            "-w" | "--week" => { target_seconds += args[i+1].parse::<u64>().expect("Failed to parse requested time for weeks") * 604800; },
            "-d" | "--day" => { target_seconds += args[i+1].parse::<u64>().expect("Failed to parse requested time for days") * 86400; },
            "-h" | "--hour" => { target_seconds += args[i+1].parse::<u64>().expect("Failed to parse requested time for hours") * 3600; },
            "-m" | "--minute" => { target_seconds += args[i+1].parse::<u64>().expect("Failed to parse requested time for minutes") * 60; },
            "-s" | "--second" => { target_seconds += args[i+1].parse::<u64>().expect("Failed to parse requested time for seconds"); },
            _ => {  }
        }
    }

    let target_time = chrono::Local::now() + chrono::Duration::seconds(target_seconds as i64);

    notif["time"] = json::JsonValue::String(format! ("{}_{}_{}_{}_{}_{}", target_time.year(), target_time.month(), target_time.day(), target_time.hour(), target_time.minute(), target_time.second()));

    // Write the notification and start the daemon
    write_notif(notif, entry_dir);
    queue_start(true, entry_dir);
}

// Check some things and start the daemon
fn queue_start(is_async: bool, entry_dir: &std::path::PathBuf)
{
    // Get the ouput of `pgrep rremind` and remove the last one (which will be this session)
    let pgrep_out = String::from_utf8(std::process::Command::new("pgrep").arg("rremind").output().expect("failed to pgrep").stdout).unwrap();
    let mut split_pgrep = pgrep_out.split("\n").filter(|i|i != &"").collect::<Vec<&str>>();
    split_pgrep.pop();

    // Kill all other instances of rremind so we don't send doubles
    for pid in split_pgrep.into_iter() { std::process::Command::new("kill").arg(pid).spawn().expect("Failed to kill existing instance of rremind!"); }

    // Depending on is_async, either start the program synchronously or asynchronously
    if is_async { async_process::Command::new("./rremind").arg("start").spawn().expect("failed to start as daemon!"); std::process::exit(0); }
    else { start_loop(entry_dir); }
}

// Start the periodic loop that sends the notifications
fn start_loop(entry_dir: &std::path::PathBuf)
{
    // Define the recurring path
    let rec_path = &mut std::path::PathBuf::from(entry_dir);
    rec_path.push("recurring");

    // Create all the dirs, just in case they don't already exist
    std::fs::create_dir_all(&entry_dir).expect("Failed to create entry directory! Do you have permission?");
    std::fs::create_dir_all(&rec_path).expect("Failed to create recurring entry directory! Do you have permission?");
    loop
    {
        // Go through each file in the config dir
        for current_entry_attempt in std::fs::read_dir(entry_dir.as_path()).unwrap()
        {
            // Make sure we want to parse this entry
            let current_entry = match current_entry_attempt
            {
                // If it's a directory, skip it. Otherwise, set the current entry to the path of the file
                Ok(..) => { 
                    let unwrapped_item = current_entry_attempt.unwrap();
                    if unwrapped_item.path().is_dir() { continue; }
                    else { unwrapped_item.path() }
                },
                // If it's any kind of error, skip the file
                Err(..) => { continue; }
            };

            // It would have continued if there was an error, so we can unwrap it freely
            // Read the entire entry to a String
            let mut current_file = std::fs::File::open(current_entry.as_path()).unwrap();
            let mut result_string = String::new();
            current_file.read_to_string(&mut result_string).unwrap();

            // Parse the contents of the entry into a json object
            let notif_read = json::parse(&result_string);
            if let Err(e) = notif_read { eprintln! ("Failed to read contents: {}", e); continue; }
            let notif = notif_read.expect("Failed to parse JSON, failed to detect error!");

            // Parse the urgency
            let req_urgency = match &notif["urgency"].as_i8().unwrap_or(1)
            {
                2 => notify_rust::Urgency::Normal,
                3 => notify_rust::Urgency::Critical,
                _ => notify_rust::Urgency::Low // Default is 1
            };

            // Get the time to notify as a chrono time
            let time_string = notif["time"].to_string();
            let time_vec = time_string.split("_").collect::<Vec<&str>>();
            let wanted_date = chrono::Local.ymd(time_vec[0].parse::<i32>().expect("Failed to parse year"), time_vec[1].parse::<u32>().expect("Failed to parse month"), time_vec[2].parse::<u32>().expect("Failed to parse day")).and_hms(time_vec[3].parse::<u32>().expect("Failed to parse hour"), time_vec[4].parse::<u32>().expect("Failed to parse minute"), time_vec[5].parse::<u32>().expect("Failed to parse seconds"));

            // Check if the time from now to the unix epoch is the same as (or greater than) the wanted time to the unix epoch
            let is_time = wanted_date.timestamp() <= chrono::Local::now().timestamp();

            // If it's time to notify, send the notification and delete the entry
            if is_time
            {
                notify_rust::Notification::new().summary(&notif["title"].to_string()).body(&notif["body"].to_string()).icon(&notif["icon"].to_string()).urgency(req_urgency).show().unwrap();
                std::fs::remove_file(current_entry.as_path()).unwrap();
            }
        }

        // Now parse all the recursive entries
        for current_entry_attempt in std::fs::read_dir(rec_path.as_path()).unwrap()
        {
            // Make sure we want to parse this entry
            let current_entry = match current_entry_attempt
            {
                // Skip the directories
                Ok(..) => {
                    let unwrapped_item = current_entry_attempt.unwrap();
                    if unwrapped_item.path().is_dir() { continue; }
                    else { unwrapped_item.path() }
                },
                // If there's an error, skip it
                Err(..) => { continue; }
            };

            // It would have continued if there was an error, so we can unwrap it freely
            // Read the entire entry to a String
            let mut current_file = std::fs::File::open(current_entry.as_path()).unwrap();
            let mut result_string = String::new();
            current_file.read_to_string(&mut result_string).unwrap();

            // Parse the contents of the entry into a json object
            let notif_read = json::parse(&result_string);
            if let Err(e) = notif_read { eprintln! ("Failed to read contents: {}", e); continue; }
            let notif = notif_read.expect("Failed to parse JSON, failed to detect error!");

            // Parse the urgency
            let req_urgency = match &notif["urgency"].as_i8().unwrap_or(1)
            {
                2 => notify_rust::Urgency::Normal,
                3 => notify_rust::Urgency::Critical,
                _ => notify_rust::Urgency::Low // Default is 1
            };

            // Set the actual times
            let time_now = [chrono::Local::now().hour(), chrono::Local::now().minute(), chrono::Local::now().second()];
            let target_time = [notif["hour"].as_u32().unwrap(), notif["min"].as_u32().unwrap(), notif["sec"].as_u32().unwrap()];

            match notif["rec_mode"].as_u32().unwrap()
            {
                1 => { // It's a daily notification
                    if target_time == time_now
                    {
                        notify_rust::Notification::new().summary(&notif["title"].to_string()).body(&notif["body"].to_string()).icon(&notif["icon"].to_string()).urgency(req_urgency).show().unwrap();
                    }
                },
                2 => { // It's a weekly notification
                    // Determine if it's the right weekday
                    let req_weekday = match notif["weekday"].to_string().to_lowercase().as_str()
                    {
                        "mon" | "monday" => chrono::Weekday::Mon,
                        "tue" | "tuesday" => chrono::Weekday::Tue,
                        "wed" | "wednesday" => chrono::Weekday::Wed,
                        "thu" | "thursday" => chrono::Weekday::Thu,
                        "fri" | "friday" => chrono::Weekday::Fri,
                        "sat" | "saturday" => chrono::Weekday::Sat,
                        "sun" | "sunday" => chrono::Weekday::Sun,
                        _ => { panic! ("Weekday not recognized!"); }
                    };
                    if target_time == time_now && chrono::Local::now().weekday() == req_weekday
                    {
                        notify_rust::Notification::new().summary(&notif["title"].to_string()).body(&notif["body"].to_string()).icon(&notif["icon"].to_string()).urgency(req_urgency).show().unwrap();
                    }
                },
                3 => { // It's a monthly notification
                    if target_time == time_now && chrono::Local::now().day() == notif["day_of_month"].as_u32().expect("Failed to parse requested day of the month!")
                    {
                        notify_rust::Notification::new().summary(&notif["title"].to_string()).body(&notif["body"].to_string()).icon(&notif["icon"].to_string()).urgency(req_urgency).show().unwrap();
                    }
                },
                4 => { // It's a weekday notification
                    if target_time == time_now && chrono::Local::now().weekday() != chrono::Weekday::Sat && chrono::Local::now().weekday() != chrono::Weekday::Sun
                    {
                        notify_rust::Notification::new().summary(&notif["title"].to_string()).body(&notif["body"].to_string()).icon(&notif["icon"].to_string()).urgency(req_urgency).show().unwrap();
                    }
                },
                _ => { panic! ("Failed to recognize recurrance mode!"); }
            }            
        }

        // Wait some time to check again
        std::thread::sleep(std::time::Duration::from_millis(950));
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
        "r" | "reccuring" => { queue_recurring(&mut entry_dir); },
        _ => { println! ("Please enter a valid mode"); }
    }
}

