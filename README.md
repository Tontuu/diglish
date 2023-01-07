# Zzep
English Cambridge dictionary scrapper but in Rust

<img src="assets/diglish_example.gif">

## Requirements
>Requires cargo
>Requires xclip to clipboard feature
>Requires notify-send functionality [notification server in unix systems e.g dunst]

## Installation
```console
$ git clone https://github.com/Tontuu/diglish.git
$ cargo build --release
$ sudo ln -s /path/to/script/target/release/diglish /usr/local/bin # Add to your preferred path.
```

## Usage
```console
diglish word [OPTIONS...]
```

### Some examples
```sh
# Help message
diglish -h

# Interactive search
diglish

# Search for word in argument
diglish dog

# Silent output
diglish kitty -q

# Copy to clipboard [needs Xclip to work]
diglish hamster -c

# Notify to desktop environment
diglish bird -n
```