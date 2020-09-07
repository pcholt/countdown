# Countdown
Unscramble the words in the TV show Countdown.

## Usage

    countdown [letters]
    
## Output

All possible words which can be created from the pattern and the dictionary from `/usr/share/dict` are written to standard output, sorted by length.
I'm sure a program like this is what they actually use in "Dictionary corner", although how they did it in the original version of the show, I have no idea.

## Improvements

This was my first effort at writing some code in `rust`. I must say, it was awfully difficult to work out the memory management system, so the point
that if you presented me with some Rust code I would have no idea if it fits Rust's memory management model.  Most of this code was throwing it at 
the wall to see if it would stick.

Very impressed with the first-class membership of testing functions, and the standard (though I didn't use it here) of defining one file for the root of the command
line application `main.rs` and another for the root of the cargo container containing the associated libraries, `lib.rs`.

I'm not sure Rust is for me, but if you want to critique this code, your criticism, patches and abuse would be invaluable.
