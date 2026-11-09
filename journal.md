# Journal

## Week 0:
Mockup UI in Figma, make presentation, do research.

- [x] Week 1: Mockup UI in Figma, research Rust libraries to be used, init repo 

## Week 1 - 3:  
Base UI and basic playback added! The way everything works basically is:
globals.slint holds some container things that have callbacks and vars. In slint I can make certain ui elements activate callbacks (eg. `clicked => { DirsState.add-directory(); }`). Then in rust I'm able to do something when that callback is activated by doing something like this: 
```
main_window.global::<DirsState>().on_add_directory({
 any code here
});
```
Right now, there's no queue, so you can just play one song and after that song is done, it stops playback (but the ui doesn't know that playback is stopped so it's actually kind of broken right now but I'll fix that later don't worry)

I'm using the crates (libraries) directories (library for cross-platform config directory management) and serde (serializing and deserializing, basically it's letting me edit the TOML config) to manage the config, which at the moment just stores your selected directories. I've tested it on my pc which runs arch linux, and my laptop which runs windows, and it works fairly well on both (while slightly laggy on linux.)

The other main crate I'm using is rodio, which is for audio playback!

The next steps for me will be ui polish, volume controls (it just blasts you at the moment), and queue implementation!
One thing that I've realized I want to add (but won't be a priority) is the ability to search through tracks, as it's rather hard to find a specific track if you have a large library at the moment.

- [x] Week 1: Mockup UI in Figma, research Rust libraries to be used, init repo 

- [x] Week 2: Rough UI skeleton, basic window/layout, placeholder views

- [x] Week 3: Implement ability to add/remove directories

- [x] Week 4: UI population

- [x] Week 5: Basic playback implementation

## Week 4:

A little less than last week (which is to be expected as last week was mostly laying down the base.); This week I mostly just polished up/animated certain UI elements and added some nice volume controls! Nothing new on the timeline is done, mostly because volume control is something I kinda forgot I would have to do at all. This isn't a huge issue though, as I'm already pretty ahead of schedule. 

No new libraries/crates were added this week.

The main next step I'm going to focus on is adding a functional queue.

- [x] Week 1: Mockup UI in Figma, research Rust libraries to be used, init repo 

- [x] Week 2: Rough UI skeleton, basic window/layout, placeholder views

- [x] Week 3: Implement ability to add/remove directories

- [x] Week 4: UI population

- [x] Week 5: Basic playback implementation

## Week 5:

Queue implemented! Essentially just changed how playback works to use a new function in the main file, which uses a audio ended callback (EmptyCallback) from rodio (which is being called from the play_file function in the player file.) to know when to start the next track. 
I also added buttons (and their associated callbacks) to allow the user to seek back and forth between tracks. The UI keeps track of which track is currently playing with an index from the tracklist, so seeking simply adds or removes one to that index.
Other than that, I gave the buttons some fancy animations! (The pause button shrinks/grows on press and hover, and the seek buttons shrink/grow as well, but also move a bit in the associated direction on press.)

Next, I'll be focusing on adding a progress bar/seeker at the bottom, and metadata scanning.

- [x] Week 1: Mockup UI in Figma, research Rust libraries to be used, init repo 

- [x] Week 2: Rough UI skeleton, basic window/layout, placeholder views

- [x] Week 3: Implement ability to add/remove directories

- [x] Week 4: UI population

- [x] Week 5: Basic playback implementation

- [x] Week 6: Basic queue implementation

## Week 6:

Progress bar :) 
The new progress bar is in the bottom bar and shows how far through the current track you are (you can also drag and click it, of course.) Basically on the math side the bar is just a number between 0 and 1, and the number is decided with position/duration. I added 2 new globals (pos and dur) and one new callback (seek). The bar only calls seek when you let go of it cause if it fired every time the mouse moved that would cause lag. 
In rust, the player in rodio has get_pos() and try_seek() so I just used those in the player file. Other than that, a slint timer is running every 100ms, and every time it fires it checks the current position and puts that in pos. The track length comes from total_duration() from the rodio decoder, which I get in play_file before the decoder goes to the player. If rodio can't find the length it just defaults to 0.

Next, I'll be doing metadata scanning.

- [x] Week 1: Mockup UI in Figma, research Rust libraries to be used, init repo 

- [x] Week 2: Rough UI skeleton, basic window/layout, placeholder views

- [x] Week 3: Implement ability to add/remove directories

- [x] Week 4: UI population

- [x] Week 5: Basic playback implementation

- [x] Week 6: Basic queue implementation

## Week 7:

Not a ton of notable progress, mostly just added the time indicator to the seek bar.

Next, I'll be doing metadata scanning.

- [x] Week 1: Mockup UI in Figma, research Rust libraries to be used, init repo 

- [x] Week 2: Rough UI skeleton, basic window/layout, placeholder views

- [x] Week 3: Implement ability to add/remove directories

- [x] Week 4: UI population

- [x] Week 5: Basic playback implementation

- [x] Week 6: Basic queue implementation

## Week 8:

Metadata scanning is done :) Tracks show their title and artist and duration now instead of the filename.

I decided to rewrite the library code instead of just modifying it, so all the new code is under librarynew.rs. (I'll probably just make it library.rs later, but this is how it is for now.)

The scanner works like this: first get_files goes through each directory (and each subdirectory) and grabs every audio file. Then for each of those files it grabs the title, artist, and duration which it grabs with read_mdata. If the file is borked it just falls back to saying the filename for the title and unknown artist for the artist and 0 for the duration. (And if there's missing metadata it just fills in the spots it can, eg. Awesome Song - Unknown Artist - 3:20 <- the artist couldn't be found) Last to_track turns the metadata into the Track struct that I use in slint, and formats the duration.  

Reading a bunch of tags EVERY time you open the app and mess with stuff is dumb, so I implemented a cache with rusqlite. It's got one table called tracks where the file path is the primary key. Each row stores the path, modified time, size, title, artist and duration. It's stored in the same folder as the config. It stores every track's path and the file size, title, artist, duration, and modified time (if you're wondering what modified time means, it's basically just a timestamp of the last time the file was changed). Each scan, stuff is only reread if it's new or it's last modified time or size changed. Tracks that are deleted are deleted from the database. It also prints a log to the console! :)

A more detailed explanation of each scan:
First the table is loaded into HashMap keyed by path, next for every file I grab the modified time and size. After that I look up the path in the map and take it out with remove (I'm not deleting it from the db, just the temporary HashMap). If the modified time and the filesize both match up I just reuse the saved metadata and if they don't or the file isn't in the map (new file) I re-read (or read) the tags and add them to the db. Finally everything that's left in the map is just deleted stuff so it deletes it from the actual db.

So on first launch/add of files it'll look like: 
365 tracks
365 read from disk
0 removed

And on the 2nd:
365 tracks
0 read from disk
0 removed

A quick explanation of how sqlite works if you don't know:
Tables are the main containers like a google sheet kinda. Then inside the tables are columns, which are like the properties of each row. And rows are basically just entries (for example in my code each track is a row). For me, I have my PRIMARY KEY on "path" which means that every row has to have a unique path, and sqlite indexes stuff off of it so it can find stuff fast without going through each row.
Then in my actual code I communicate with the db with the language SQL. (eg. SELECT to read rows, INSERT OR REPLACE to add a track (or overwrite), and DELETE.)
The main reason I'm using this over a JSON or TOML or something is cause it's more efficient to use sqlite (and faster with huge libraries). With something like JSON I'd have to rewrite everything for every change (as opposed to sqlite only changing the rows that actually changed).

New crates this week: rusqlite, lofty
 
Next up: OS integration!

- [x] Week 1: Mockup UI in Figma, research Rust libraries to be used, init repo 

- [x] Week 2: Rough UI skeleton, basic window/layout, placeholder views

- [x] Week 3: Implement ability to add/remove directories

- [x] Week 4: UI population

- [x] Week 5: Basic playback implementation

- [x] Week 6: Basic queue implementation

- [x] Week 7: Metadata scanning

## Week 9:

After research, I've decided against OS integration. I don't use it that much personally, and it seems like it'll be a pain to implement (the crate I was planning on using is not actively maintained since 2025, there are weird cross OS dependencies, etc.) 

Instead, this week, I added keyboard controls by wrapping the main window in a FocusScope and adding various binds (eg. space to pause, arrow keys to seek, arrow keys + modifiers to skip.)

Next, I'll be doing settings polish.

- [x] Week 1: Mockup UI in Figma, research Rust libraries to be used, init repo 

- [x] Week 2: Rough UI skeleton, basic window/layout, placeholder views

- [x] Week 3: Implement ability to add/remove directories

- [x] Week 4: UI population

- [x] Week 5: Basic playback implementation

- [x] Week 6: Basic queue implementation

- [x] Week 7: Metadata scanning
