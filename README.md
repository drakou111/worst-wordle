# Worst Wordle

## What is this?
This repository contains code for finding all wordle games that could lead to all tiles being grey. However, you are not allowed to use gray letters in your following guesses. For example, if I guess "house" and all tiles are gray, I won't be able to use any of those letters in my next guess. If you get a green tile, you are forced to use it. If you get a yellow tile, you are also forced to use it, but can't use it in a spot that was already checked (for example, if I get a yellow tile 'S' at position 2, I can't put a yellow tile 'S' at that position again).

The goal of this project was to simply find all possible games of Wordle where this is possible (ignoring duplicates).

This idea came from Ellie  Hasmussen, in their video [You should play Wordle wrong](https://www.youtube.com/watch?v=zoh5eLOjwHA), which tried to solve this idea.

## Format
"solutions" contains every single solution for every single word, for a total of 2,262,234 *unique* solutions. "seven" contains all 7 solutions for the version where you get 7 guesses instead of 6 (I was quite surprised to see this result).

When I say a "unique" solution, it means that it accounts for all guess orders possible (guess "house" then "apple" is identical to guessing "apple" then "house" in the context of Worst Wordle, and also accounts for the fact that multiple words can act the same. For example, "erode", "odder", "rodeo" and "order" all contain the same letters ('e', 'r', 'o' and 'd'), so a solution will pretend that all of those is only one word.

Solutions are in the format of something like:
"cacao/cocoa -> grrrl, pfftt, vivid, exeem/exeme, wynns, zhuzh". / means that it could be either word (since they contain the same letters), and is formatted like: [answer] -> [guess1], ..., [guess6]. Folder names represent the answer of the board, with a "_" between each word if it has multiple equivalent words.
## How does it work?

### Binary Representation

This is a single rust file that  does some pruning and bit operations to speed things up. First thing is that each word is represented as a 32bit integer. The first bit represents whether or not the word contains an 'a'. The second bit represents 'b', etc.

The board state can also be represented as a single integer. When we add a guess to the board, we can just use a bitwise OR operation to combine them.

To check if our word "fits" (AKA, doesn't contain any letters in previous guesses/answer), it simply comes down to a bitwise AND operation, and checking if the result is == 0. If so, that means no letter in our guess was in the board state.

### Pruning

There are two major pruning techniques. The first is to realize that we don't need to use the 2000+ words list each guess, we can use the list of words that was found from the previous guess. For example, if the previous guess found that only 1000 words could be fit as a first guess, then we know that for the second guess, there can be at most those 1000 words that can fit.

The last one is to sort all words in their integer format from lowest to highest. Then, when we do a guess and it happened to be a word with the number 9182, then we know for a fact that the next guess must be >9182, because any word under that will lead into a duplicate case we've already seen before. This way we only get unique and sorted solutions. You can see this by noticing all solutions in "solutions" tends to go from a-z.

## Other info
### What's the strategy to win in a real game?
If you want to try winning in an actual game, these are the best solutions (they each work for 10 answers) (odds of getting all grey in a random game is 0.432%):

[pfftt, urubu, weeke, xviii, gynny, zocco]
works for answers: [amass, llama, madam, mamma, salad, salsa, shall, slash, small, smash]

[qajaq, pfftt, susus, xviii, gynny, zorro]
works for answers: [beech, belch, belle, bleed, cheek, check, embed, emcee, leech, melee]

[pfftt, susus, mahwa, xviii, gynny, jeeze]
works for answers: [block, blood, brood, brook, clock, color, crock, crook, droll, drool]

### What words are required to make wrong wordles possible?
If none of these words were included in the allowed words: [grrls, cwtch, phpht, pfftt, crwth, grrrl], then it would be impossible to get a fully gray wordle.

## Why?
felt like it