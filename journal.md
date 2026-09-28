# Journal

## Week 1 (and kind of week 0 as well, sorta):  
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
