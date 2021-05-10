# Simple Rust script for weighted sampling from a text file

Reads a file line-by-line, each line formatted as `<integer> <string>`. Loops N times,
printing the string from a randomly-chosen line, biased using the integer as a weight.
Does not output the same value twice in a row.

## Why?

Because I know Rust better than I know bash / unix commands. :laughing:

Took me less time to write this, than it would have taken me to figure out the
magic incantation of unix commands to do the same thing.

## Example

For a while, I used this to [gamify](https://en.wikipedia.org/wiki/Gamification)
learning languages. I made a list of languages I was interested in improving in,
along with weights, to bias the selection towards those that were most
important to me. Every day, I would run this program to choose a foreign
language that I would spend some time on during that day.

Here is the file I used:

```
5 dansk
3 español
1 українська
1 toki pona
1 esperanto
2 deutsch
3 русский
2 arabic
1 mandarin
1 nihongo
```

(because i am crazy and i can totally juggle learning like 10 foreign languages
all at once :wink: )

And the command:

```shell
randsample langs 7
```

Make a learning programme for the week, with a different language for each day.

