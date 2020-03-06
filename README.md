# Ranki
A barebones Anki-inspired tool written in <a href="https://www.rust-lang.org/">Rust</a>.

## What's Anki?
<a href="https://apps.ankiweb.net/">Anki</a> is a spaced-repetition flashcard application
that is designed to help retain information by prompting you with the card
containing the information right as you are predicted to forget it. The algorithm
guiding Anki is the
<a href="https://www.supermemo.com/en/archives1990-2015/english/ol/sm2">SuperMemo 2</a>
algorithm, the last iteration of the SuperMemo algorithm that is permissively
licensed. Anki's implementation differs somewhat from this algorithm, and Ranki
differs a good bit from Anki's implementation.

## Why Ranki?
- I wanted a fun Rust project!
- I've wanted to start regularly using a spaced repetition tool for a while now,
so why not make my own?
- Straightforward feature set, just manage flashcards and quiz yourself. Making
use of the <a href="https://en.wikipedia.org/wiki/Pareto_principle">Pareto Principle</a>
here.

## Building and Running
This application is written using <a href="https://github.com/antoyo/relm">Relm</a>,
a delightful little library giving you a nice MVC, Elm-inspired wrapper on top
of Rust's <a href="https://gtk-rs.org/">GTK bindings</a>. As such, you'll need
to make sure to install the
<a href="https://www.gtk.org/docs/installations/">GTK development dependencies</a>,
and of course the <a href="https://www.rust-lang.org/tools/install">Rust toolchain</a>
and then you should be one `cargo build --release` away from an eidetic memory.

## Modifying the Scheduling Algorithm
The scheduling logic can be found in `schedule.rs`, so feel free to put your own
spin on it and do something you're happy with. I'm still tweaking it for my own
purposes, so whatever is in there now certainly isn't gospel.

## Disclaimer
This project is a very alpha experience, so there may be a couple rough edges
lurking in there somewhere. Feel free to log an issue in the tracker if you find
one!
