# Day 7: Amplification Circuit

rewritten the IntCode Interpreter to use `use std::sync::mpsc::{Receiver, Sender}`. 
The program spawns 5 threads for the amplifiers and wires the respective channels together.
For part 2 the programs ends when the channel of amplifier A is closed.

## Part 1
```
$> cargo run input 0 4 | sort -n
...
398674
```

## Part 2
```
$> cargo run input 5 9 | sort -n
...
39431233
```
