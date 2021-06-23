extern crate notify_rust;

use notify_rust::Notification;

fn main()
{
    Notification::new().summary("Title here").body("This is the body of the notification").icon("/home/jake/downloads/ogayu.jpg").show().expect("Some sort of error occurred!");
}
