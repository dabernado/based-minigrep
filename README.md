# based-minigrep
based-minigrep is my implementation of the 'minigrep' coding challenge in the Rust book, but with a few key differences:

	* Uses "match" in environment argument parsing, in order to send error messages when there are not enough arguments AND too many
	* Implements the "case_sensitive" environment variable as a flag passed to the program (as "-c", passed as the last argument to the program)
		* The flag is also handled using "match", enabling easy addition of new flags
	* Finds words in text using a regex, instead of a string slice, for better robustness
	* Matches only if the substring in the text is the query as a full word (ex., 'the' will match 'the' but not 'there')
	* Returns the number of occurences of the query, rather than the lines which contain them (because I intend for this code to be used in a larger project, which doesn't care about lines of text)

Features I may add in the future, but probably won't:

	* Returns the indices of each occurence of the query
	* More flags
	* Can place flag anywhere in the program call, instead of forcing it to be the last argument

## I want this to be a library!!
Sure, just use 'grep.rs' from the 'src' directory and it should give you everything you need. Keep in mind that it is still licensed under the AGPLv3.
