# xtcat: Fast XTC files concatenator

`xtcat` is to be used **only** for concatenating xtc files that come from simulation runs directly following each other. `xtcat` is actually just a slightly smarter `cat` as it stitches all xtc files together while **removing the first frame of the trajectory from every xtc file except for the first one**. In other words, `xtcat` assumes that the first frame of the concatenated trajectory is the same as the last frame of the previous trajectory (but does not check in any way whether this is actually true) and removes it to avoid duplicate frames.

## Installation

0) Install [rust](https://www.rust-lang.org/tools/install).
1) Run `cargo install xtcat`.


## Example usage

```
xtcat -f md0001.xtc md0002.xtc md0003.xtc -o md_cat.xtc
```

The program will concatenate files `md0001.xtc`, `md0002.xtc`, and `md0003.xtc` and write the output into `md_cat.xtc`. The first trajectory frame from `md0002.xtc` and `md0003.xtc` will be removed from the output. The input files will NOT be sorted in any way, neither by their name nor by their starting time. The input files will not be modified.

Use flag `-s` if you do not want `xtcat` to print to standard output. Errors will still be printed into `stderr`.

## Limitations

Only tested on Linux but should work anywhere.
