# rremind
rremind is a reminder program written in rust (hence the name). Once complete, it will allow the user to utizile four main features; scheduling a notification a defined amount of time in the future, scheduling a notification to be sent on a certain date/time, scheduling a recurring notification, either daily, weekly or monthly, and daemon mode.

# Contents
- [Installation](#Installation)
- [How to Use](#How-to-Use)
- [Contact Me](#Contact-Me)

# Installation

There are currenly no official releases. It will likely be a little while before there are, since I'm not familiar with the best ways to package rust/cargo programs for distrobution. For the time being, I would recommend downloading the source code and building it with `cargo build`, then putting `target/debug/rremind` somewhere in the system path.

# How to Use

As mentioned above, there are four main ways to use rremind; Instant mode, Single mode Recurrance mode and Daemon mode.

### Daemon Mode

Daemon mode starts the program in the background, checking the queue every second for whether or not to send a notification. A notification will be sent if the specified time is or is before the current system time. Non-recurrant entries will be deleted as soon as the notification is sent. Once started in daemon mode, all other instances of rremind are killed. All entries are stored in `~/.local/share/rremind/`. Finally, note that rremind's daemon mode isn't technically a daemon, but I couldn't think of any better way to describe it.

### Instant

Instant mode schedules a notification to be sent out in a specified amount of time - rather than saying "I would like to be notified at this time", you say "I would like to be notified in this long". Due to the nature of the purpose of this mode, the daemon is started as soon as the entry is added to the queue, something unique to this mode. To use instant mode, you can call `rremind add i [args]`. As with the other add modes, you can use `-t` to specify a title, `-b` to specify a body, `-i` to specify an icon, and `-u` to specify an urgency between 1 and 3.

The countdown, specified by `-c` is currently outdated, and while it works, it is confusing and will be replaced soon. To specify the amount of time, provide an argument in the form `ni-ni-ni...`, where `n` is a number and `i` is the time unit indicator (`w` for weeks, `d` for days, `h` for hours, `m` for minutes, `s` for seconds). The same time unit indicator can be used multiple times, and they will be added together.
```bash
# Send a notification with the specified fields in 2 weeks, 4 days, 1 hour, 33 minutes and 14 seconds
rremind add i -t "This is my title" -b "this is my body" -i ~/path/to/icon -u 2 -c 2w-4d-1h-33m-14s
```

### Single

Single mode schedules a notification to be sent out at a specified time. Unlike Instant mode, this mode only adds the entry to the queue, it does not start the daemon - you need to do that yourself before the notification is scheduled in order to receive it on time. To use single mode, you can call `rremind add s [args]`. As with the other add modes, you can use `-t` to specify a title, `-b` to specify a body, `-i` to specify an icon, and `-u` to specify an urgency between 1 and 3.

The time specification method, unlike instant mode, makes sense. Use `-y` to specify the year (A.D.), `-o` or `--month` to specify the month, `-d` to specify the day, `-h` to specify the hour, `-m` or `--minute` to specify the minute and `-s` to specify the second. Any fields not specified are assumed to be the same as the current system time of that field.
```bash
# Send a notification with the specified fields on August 3rd, 2021 at 5:30 PM, with the seconds the same as the current system time's seconds
rremind add s -t "This is my title" -b "this is my body" -i ~/path/to/icon -o 8 -d 3 -h 17 -m 30
# Don't forget to start the daemon
rremind start
```

### Recurrance

There is currently no recurrance mode.

# Contact Me

If you have any feedback, suggestions, errors or just general comments, please email me at Jake_Guy_11@protonmail.ch, message me at JakeGuy11#1541 on Discord, or open an error through GitHub.