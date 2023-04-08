# Robotic Utility Barely Beneficial In Serving Humans
## Statistical text generator

This Program generates text based on patterns in an analyzed corpus.

## Usage
On first execution, RUBBISH will create a `./corpora` folder. There, you should create a new folder with a name of your corpus containing .txt files. When this folder is chosen, RUBBISH will create `./corpora/your_corpus_name_/data` folder, where it will place cached files. RUBBISH will then tokenize the corpus and analyze patterns in it. After this you can input text which will then be continued. 

## Example
(this example uses the wikipedia corpus from https://www.english-corpora.org)

File structure before:
```
parent/
├─ corpora/
│  ├─ gutenberg/
│  │  ├─ An ethical philosophy of life presented in its main outlines - Felix Adler.txt
│  │  ├─ The Dance of Life - Havelock Ellis.txt
│  │  ├─ The Price of a Soul - William Jennings Bryan.txt
│  │  ├─ The Sixth Sense - Its Cultivation and Use - Charles Henry Brent.txt
```

RUBBISH run:
```
<0.00> Choose corpus:
0: gutenberg
     > 0    
<3.09> Input new tokens count:
     > 20000
<5.48> Input phrase length:
     > 3
<9.87> found previously created tokens at ./corpora/gutenberg/data/20000_tokens.bin, loading...
<9.88> tokenizing whole corpus
<9.95> Error while tokenizing: 'unknown token in input: '’''. Exiting...
(base) PS D:\pliki\programowanie\rust\projs\commandline\rubbish\release> .\rubbish.exe
<0.00> Choose corpus:
0: gutenberg
     > 0
<1.87> Input new tokens count:
     > 20000
<4.91> Input phrase length:
     > 3
<7.37> reading corpus and extracting word frequencies...
<7.51> producing starting tokens...
<7.54> tokenizing words...
<7.58> generating new tokens...
▏▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▕ 20000/20000
<102.51> example words:
[7230, 2101] = |“histor|y,” |: 1
[1911, 4923] = |mechan|ics. |: 1
[1499, 14679] = |(_|chapters |: 1
[45, 131, 165] = |r|ar|er |: 1
[846] = |d. |: 1
[3913, 165] = |seren|er |: 1
[18830] = |ethics_. |: 1
[6771] = |sly |: 1
[11273] = |naturalistic |: 2
[7693] = |prediction |: 3

<102.51> saving vocabulary to ./corpora/gutenberg/data/20000_tokens.bin
<102.52> tokenizing whole corpus
▏▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▕ (585529/585529) tokenizing: An ethical philosophy of life presented in its main outlines - Felix Adler.txt
▏▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▕ (20000/20000) merging: An ethical philosophy of life presented in its main outlines - Felix Adler.txt
▏▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▕ (561323/561323) tokenizing: The Dance of Life - Havelock Ellis.txt
▏▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▕ (20000/20000) merging: The Dance of Life - Havelock Ellis.txt
▏▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▕ (52653/52653) tokenizing: The Price of a Soul - William Jennings Bryan.txt
▏▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▕ (20000/20000) merging: The Price of a Soul - William Jennings Bryan.txt
▏▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▕ (109450/109450) tokenizing: The Sixth Sense - Its Cultivation and Use - Charles Henry Brent.txt
▏▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▕ (20000/20000) merging: The Sixth Sense - Its Cultivation and Use - Charles Henry Brent.txt
<218.24> saving tokenization to ./corpora/gutenberg/data/20000_tokenization.bin
<218.25> generating succession tree...
▏▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▕
<234.48> saving succession tree to ./corpora/gutenberg/data/20000_3_tree.bin
<234.52> Text to be continued:
       > A man can 
<241.40> a man canon told baretti in 1770, in spite of its occasional indecorum, as a useful safety-valve for the emotions. it is clear that his empirical conception of the family as serving a larger purpose is disappearing, and that the sequence of phenomena observed by him. and the same old combinations inevitably tend to recur. but that notion scarcely fits all the facts, and for all sorts of antique metaphysical peculiarities, inherited from the decadence of greek philosophy, were attributed to “spirit”; “matter” played the devil’s part to this more divine “spirit.” thus it was that i now possessed a second object, namely, the laborer, to whom i could apply my non-violation ethics. its formula is: treat man never merely as a means, and what the relation of cause and its effect, that is declared to be necessary, but the sequence of b on a, the circumstance that in proportion as it is ever likely to be, its critics were bemoaning its corruption, lamenting, for instance, the right which is commonly designated as “the freedom of conscience.”
```

File structure after
```
parent/
├─ corpora/
│  ├─ wikipedia/
│  │  ├─ An ethical philosophy of life presented in its main outlines - Felix Adler.txt
│  │  ├─ The Dance of Life - Havelock Ellis.txt
│  │  ├─ The Price of a Soul - William Jennings Bryan.txt
│  │  ├─ The Sixth Sense - Its Cultivation and Use - Charles Henry Brent.txt
│  │  ├─ data/
│  │  │  ├─ 20000_tokens.bin
│  │  │  ├─ 20000_tokenization.bin
│  │  │  ├─ 20000_3_tree.bin
```

## Limitations
RUBBISH is known to straight up copy fragments of the corpus. Lower phrase length and larger corpora make this less likely.