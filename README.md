# todo-app

This repo is mostly just to track self-learning progress in Rust

Following instructions from git@tsoding for Rust and NeuralNine for Python

## Simple Terminal Todo app in Python

- library used: click
- no UI implemented
- basic commands only

## Interactive Terminal Todo app in Rust

- simple UI implemented for interacting within Terminal

### Run in rust directory

```console
    cargo run TODO
```

### Controls

|Keys|Description|
|---|---|
|<kbd>w</kbd>, <kbd>s</kbd>|Move cursor up and down|
|<kbd>Tab</kbd>|Switch between the TODO and DONE panels|
|<kbd>q</kbd>|Quit|
|<kbd>Shift+W</kbd>, <kbd>Shift+S</kbd>|Drag the current item up and down|
|<kbd>g</kbd>, <kbd>G</kbd> | Jump to the start, end of the current item list, to be implemented|
|<kbd>r</kbd>|Rename the current item, to be implemented|
|<kbd>i</kbd>|Insert a new item, to be implemented|
|<kbd>d</kbd>|Delete the current item, to be implemented|
|<kbd>Enter</kbd>|Perform an action on the highlighted UI element, to be implemented|
|<kbd>Num1-5</kbd>|Perform an action to mark priorities' colors of items (TODO), to be implemented|

All credits to @tsoding for live session co-coding
