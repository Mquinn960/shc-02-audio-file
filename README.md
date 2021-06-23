# Create an audio file with sound

The goal of this project is to create an audio file with any playable sound included.
### MVP

- Produce any sort of audio file which is playable by something
### Bonus Points

- Play the sound using your creation
- Ability to change sound via input
### Alloted Time

|Phase|Allowed Time|
|---|:---:|
|Planning|00:20:00|
|Build|04:00:00|

## Approach
### Design

![Alt text](/docs/shc-02-audio.jpg?raw=true "whiteboard planning")

I started out with the intention to create a .wav file, knowing that this is a raw file type and I
wouldn't have to faff about with compression or parity bits. In general I knew how PCM is used to 
sample an analogue signal but knew nothing about the .wav format's constituent parts or how to 
write one.

I did some research on these files, the headers, the data etc. and added the stretch goal of adding
multiplexing (stereo audio) to the resultant file.

### Implementation

As this was a low-level engineering task, I wanted to try out Rust, which I have no previous experience with.

Mostly, this project just involved writing various standard bytes to a file, then streaming in the appropriate
data to produce a sound when passed into an audio program capable of parsing a .wav file.

The program itself is simple, mostly self-contained in `main.rs`, but I wanted to explore how to structure a 
Rust program and so burnt some time working out how to separate files and import them correctly.

Eventually I also added a crate from `dasp` which is an excellent Rust Digital Signal Processing (DSP) library
which saved having to manually generate a sine wave oscillator.

The resultant program can generate a distinct sine wave tone at a variable frequency and duration, which can be
correctly processed by any audio program conversant with .wav files.

---
## Evaluation

Overall, I feel this project went well. I havn't done any low-level work for a while and it
took some time to get into working so close to primitive data types again.

The program itself works and is succinct. Looking at the timings I definitely lost plenty of time
just getting going with Rust, setting up the project initially, but that's to be expected having
not used it before.

I'm glad I got this working as I probably had about 10 minutes to spare by the time I actually
got a usable tone. In the case of PCM, it was quite thrilling to work out why writing the same
bits over and over again produces no tone (there is no delta in signal samples).

Structurally the program is probably poor, and really doesn't make use of options/traits -
because I don't really understand how these work yet - but I'm curious to learn more.

### Timings

|Phase|Allowed Time|
|---|:---:|
|Planning|00:15:45|
|How do I Rust|00:10:09|
|Structure and naming|00:07:14|
|Creating file header structs|00:09:41|
|Reading WAV spec|00:07:28|
|Working out module imports|00:06:44|
|File writing|00:22:31|
|Panic party|00:04:28|
|String conversions, structs, scope|00:35:17|
|Creating example wav file|00:02:20|
|File value defaults|00:11:08|
|Generating audio|00:06:50|
|Writing main file blocks|01:12:25|
|Option and Unwrap?|00:05:51|
|Trying dasp|00:27:21|
|It beeps!|00:01:09|
|Rapid tidy up|00:08:15|

Total build time: 3:58:51

### Lessons Learned

- We often take primitve types for granted, it's worth stopping and thinking about encodings,
  bytes, buffers and what the OS is doing
- Rust was enjoyable but mysterious
- Understanding the domain is always important

## Credits

With thanks to the below authors/sources

Links:

- https://sites.google.com/site/musicgapi/technical-documents/wav-file-format
- https://github.com/Thrifleganger/audio-programming-youtube/blob/master/wav-file-format/main.cpp
- https://gist.github.com/jimmychu0807/9a89355e642afad0d2aeda52e6ad2424
- https://stevedonovan.github.io/rust-gentle-intro/2-structs-enums-lifetimes.html
- https://stackoverflow.com/questions/28137559/can-someone-explain-wavwave-file-headers
- http://www.topherlee.com/software/pcm-tut-wavformat.html