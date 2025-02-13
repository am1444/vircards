# vircards
Simple virtual flashcards for studying purposes.

**Use**
vircards FILE: File is the path of the cardfile.

**Preparing Cards**
Flashcards are stored in a cardfile. A cardfile is a simple plaintext file used to store information about cards that you can create with a text editor. The following is an excerpt from an example cardfile:

_La computadora portátil
Laptop
BOTHDIR

Descargar
To download
FORDIR_

The format is as follows:

_TERM
DEFINITION
DIRECTION_

In which:

-Term is the term that is being studied, like on the front of a flashcard.
-Definition is the definition of the term, like on the back of a flashcard.
-Direction can be either "BOTHDIR" or "FORDIR". "BOTHDIR" means that the user will be told the term or definition and then asked for the other one. "FORDIR" means that the user will only ever be given the term and then asked for the definition.

# Compile
1: Make sure that you have _cargo_ installed and the latest version of the rust toolchain (via rustup).
2: `cd` into the parent directory (the directory containing this folder).
3: Use `cargo make` to produce a binary at `./target/debug/vircards`. Move `vircards` into somewhere inside your shell's $PATH.
