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
