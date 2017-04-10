Scrabble Challenge - Cosmologicon https://www.reddit.com/r/dailyprogrammerhttps://www.reddit.com/r/dailyprogrammer/comments/5go843/20161205_challenge_294_easy_rack_management_1/comments/5aemnn/20161031_challenge_290_easy_kaprekar_numbers/

Challenge: Given an input of available characters and an output word, check if the word can be created with the available characters. 

Eg. scrabble("ladilmy", "daily") -> true
    scrabble("eerriin", "eerie") -> false

Bonus 1: Handle blank tiles (represented by "?"). These are "wild card" tiles that can stand in for any single letter.

Eg. scrabble("pizza??", "pizzazz") -> true
    scrabble("piizza?", "pizzazz") -> false
