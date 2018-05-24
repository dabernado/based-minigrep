# based-minigrep
based-minigrep is my implementation of the 'minigrep' coding challenge in the Rust book, but with a few key differences:

- Can search for multiple words in a file at once via multithreading
- Uses "match" in environment argument parsing, in order to send error messages when there are not enough arguments AND too many
- Uses the newer, simpler "fs::read_to_string" function over using "File::open", etc.
- Implements the "case_sensitive" environment variable as a flag passed to the program (as "-c", passed as the last argument to the program)
	- The flag is also handled using "match", enabling easy addition of new flags
- Finds words in text using a regex, instead of a string slice, for better robustness
- Matches only if the substring in the text is the query as a full word (ex., 'the' will match 'the' but not 'there')
- Returns the number of occurences of the query, rather than the lines which contain them (because I intend for this code to be used in a larger project, which doesn't care about the lines of text)

Program calls are structured like this:

```
based-minigrep [filename] [-c | -n] [queries]
```

Here is an example:

```
based-minigrep poem.txt -c the you
```

Features I may add in the future, but probably won't:

- Returns the indices of each occurence of the query
- More flags

## Build and Install

```
git clone https://github.com/davidabernado/based-minigrep
cd based-minigrep
cargo build --release
```

 Once the binary is built, move it to /usr/bin/ or wherever you store your binaries.

## I want this to be a library!!
Sure, just use 'grep.rs' from the 'src' directory and it should give you everything you need. Keep in mind that it is still licensed under the AGPLv3.
