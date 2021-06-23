extern crate notify_rust;

use notify_rust::Notification;

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
        "i" => { println! ("You've selected instant mode"); },
        "r" => { println! ("You've selected reccurant mode"); },
        _ => { println! ("Please enter a valid mode"); }
    }

    // Notification::new().summary("Title here").body("This is the body of the notification").icon("/home/jake/downloads/ogayu.jpg").show().expect("Some sort of error occurred!");
}
