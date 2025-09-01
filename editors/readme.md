# PianoHamster Syntax for VS Code


Minimal syntax highlighting for  PianoHamster language.


## Features
- Highlights keywords (tempo, time, key, instrument, velocity, voice, note, chord, rest, pedal)
- Highlights pitches, durations, bars, and comments


## Installation
- Open VS Code > `F1` > `Developer: Install Extension from Location...` and select the folder.
- Or package with `vsce` and install the `.vsix`.


## Packaging
```bash
npm i -g vsce
vsce package
# Produces pianohamster-syntax-0.0.1.vsix
