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
```bash
rremind start
```

### Instant

Instant mode schedules a notification to be sent out in a specified amount of time - rather than saying "I would like to be notified at this time", you say "I would like to be notified in this long". Due to the nature of the purpose of this mode, the daemon is started as soon as the entry is added to the queue, something unique to this mode. To use instant mode, you can call `rremind add i [args]`. As with the other add modes, you can use `-t` to specify a title, `-b` to specify a body, `-i` to specify an icon, and `-u` to specify an urgency between 1 and 3.

The time specification method lets you specify how far out you would like the notification by specifying specific time flags. Use `-w` to specify the number of weeks, `-d` to specify the number of days, `-h` to specify the number of hours, `-m` to specify the number of minutes, and `-s` to specify the number of seconds.
```bash
# Send a notification with the specified fields in 2 weeks, 4 days, 1 hour, 33 minutes and 14 seconds
rremind add i -t "This is my title" -b "this is my body" -i ~/path/to/icon -u 2 -w 2 -d 4 -h 1 -m 33 -s 14
```

### Single

Single mode schedules a notification to be sent out at a specified time. Unlike Instant mode, this mode only adds the entry to the queue, it does not start the daemon - you need to do that yourself before the notification is scheduled in order to receive it on time. To use single mode, you can call `rremind add s [args]`. As with the other add modes, you can use `-t` to specify a title, `-b` to specify a body, `-i` to specify an icon, and `-u` to specify an urgency between 1 and 3.

The time specification method lets you specify the specific datetime parameters of when you would like the notification. Use `-y` to specify the year (A.D.), `-o` or `--month` to specify the month, `-d` to specify the day, `-h` to specify the hour, `-m` or `--minute` to specify the minute and `-s` to specify the second. Any fields not specified are assumed to be the same as the current system time of that field.
```bash
# Send a notification with the specified fields on August 3rd, 2021 at 5:30 PM, with the seconds the same as the current system time's seconds
rremind add s -t "This is my title" -b "this is my body" -i ~/path/to/icon -o 8 -d 3 -h 17 -m 30

# Don't forget to start the daemon
rremind start
```

### Recurrance

Within recurrance mode, there are 4 sub-modes, each indicating how often you would like the notifications; daily, weekdaily, weekly and monthly. The standard notification fields are available (`t` to specify a title, `-b` to specify a body, `-i` to specify an icon, and `-u` to specify an urgency between 1 and 3). However, unlike other modes, recurrance mode requires another argument, the fourth argument (including the rremind binary) must indicate which sub-mode to use; `wd` for weekdaily, `d` for daily, `w` for weekly, and `m` for monthly. Every sub-mode needs the time of day to be specified, using `-h` for hour, `-m` for minute, and `-s` for second. Reccurance mode does not start the daemon automatically.

#### Weekdaily/Daily

Weekdaily and daily do not need anything special - they send a notification every day (for weekdaily, skipping Saturday and Sunday) at the specified time.
```bash
# Send a notification every monday through friday at 10:30 AM
rremind add r wd -t "This is my title" -b "this is my body" -i ~/path/to/icon -h 10 -m 30

# Send a notification every day at 10:30 AM
rremind add r d -t "This is my title" -b "this is my body" -i ~/path/to/icon -h 10 -m 30

# Don't forget to start the daemon
rremind start
```

#### Weekly

Weekly requires that you provide the day of the week (using `-w`) along with the time to send it. It recognizes `mon`/`monday`, `tue`/`tuesday`, `wed`/`wednesday`, `thu`/`thursday`, `fri`/`friday`, `sat`/`saturday`, and `sun`/`sunday`. It is **not** case sensitive.
```bash
# Send a notification every tuesday at 6:00 PM
rremind add r w -t "This is my title" -b "this is my body" -i ~/path/to/icon -h 18 -w tue

# Don't forget to start the daemon
rremind start
```

#### Monthly

Monthly requires that you provide the day of the month (using `-d`) along with the time to send it. It accepts only a number as the day - there are no generic terms for things like "last day of the month". 
```bash
# Send a notification on the 11th pf every month at 10:00 PM
rremind add r w -t "This is my title" -b "this is my body" -i ~/path/to/icon -h 22 -d 11

# Don't forget to start the daemon
rremind start
```

# Contact Me

If you have any feedback, suggestions, errors or just general comments, please email me at Jake_Guy_11@protonmail.ch, message me at JakeGuy11#1541 on Discord, or open an error through GitHub.
