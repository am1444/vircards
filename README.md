# vircards
Simple virtual flashcards for studying purposes.

**Use**
vircards FILE: File is the path of the cardfile.

The program will prompt the user and allow the user to type a response. Then the program will tell the user the correct answer and the user will decide if they have gotten it right. If they decide that they responded correctly, then they must type '`g`' (for "good") to tell `vircards` that they have mastered that card. Then, the user will not be prompted with that card again. When the user has mastered all the cards, the program will congratulate them and then it will exit.

**Preparing Cards**
Flashcards are stored in a cardfile. A cardfile is a simple plaintext file used to store information about cards that you can create with a text editor. The following is an excerpt from an example cardfile:

_La computadora portátil_

_Laptop_

_BOTHDIR_


_Descargar_

_To download_

_FORDIR_

The full example cardfile is in this repo as `cardfile.txt`. The format is as follows:

TERM

DEFINITION

DIRECTION

In which:

-Term is the term that is being studied, like on the front of a flashcard.

-Definition is the definition of the term, like on the back of a flashcard.

-Direction can be either "BOTHDIR" or "FORDIR". "BOTHDIR" means that the user will be told the term or definition and then asked for the other one. "FORDIR" means that the user will only ever be given the term and then asked for the definition.
    Note: If neither FORDIR nor BOTHDIR are present in a single card, the default behaviour is FORDIR.

# Compile
1: Make sure that you have _cargo_ installed and the latest version of the rust toolchain (via rustup).

2: `cd` into the parent directory (the directory containing this file).

3: Use `cargo build` to produce a binary at `./target/debug/vircards`. Move `vircards` into somewhere inside your shell's $PATH.

# Exiting
Press ctrl+c (^C) to exit.
