# Many Time Pad

Keys in One-time pad encryption (OTP) should only be used once, when they get reused we can do a Many-time pad attack.

MTP Interactive uses automated cryptanalysis to present a partial decryption which can be solved interactively.

It was created inspired by the cryptography course by Standford University on Coursera.

This is a fork of [MTP](https://github.com/CameronLonsdale/MTP) rewritten in Rust for fun.

## Install

Make sure you have Rust installed on your machine. Then run:

```
cargo build --release
```

## Usage
Run the file located at ./target/release/many-time-pad
```
many-time-pad --file examples/sample.ciphertexts
```

[![asciicast](https://asciinema.org/a/204705.png)](https://asciinema.org/a/204705)

### Intstructions

Cursor movement is similar to Sublime Text:
 - Left, Right, Up and Down for simple movement
 - Home, End, Page Up and Page Down for larger movement
 - Left Click for jumping to mouse cursor

Letters can be entered using the keyboard any time.

Press ESC to exit

### Notes

The current application still under development so a few things are not ready yet. 
Feel free to create an issue for any bug or suggestion.
