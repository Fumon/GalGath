# GalGath

GalGath (GalacticGatherer) is a small service which makes reading webnovels posted on Reddit and Patreon easier by converting them to EPub and serving/sending those EPubs to supported e-readers.

The goal is to make sending a new chapter from your phone to your e-reader a three click or less operation.

## Rationale

### Why?

I read a bunch of [/r/HFY](https://www.reddit.com/r/HFY/) and follow several series from there on Patreon, but really dislike the Patreon reading interface on mobile. I have a perfectly good e-reader (a Kobo Forma) which has been gathering dust lately, but which has a wifi connection and a primitive web browser which I've used in the past for similar things.

### How?

I'm using [crowbook](https://github.com/lise-henry/crowbook) as a library to do the markdown to epub conversion.

## Features

- Reddit
  - [ ] Insert link, get epub
- Patreon
  - [ ] Authentication
  - [ ] Insert link, get epub
- Series Tracking
  - [ ] Puts multiple chapters together in order

### Series Tracking

This is a stretch feature for sure and may come in a different form altogether.

Some ideas I've been playing with involve a small chat server with channels for different series.
