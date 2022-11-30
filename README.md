# xtcat: Fast XTC Files Concatenation

`xtcat` is a simple and fast concatenator of xtc files. It is to be used **only** for concatenating xtc files that come from simulation runs directly following each other. `xtcat` is actually just a slightly smarter `cat` as it stitches all xtc files together while **removing the first frame of the trajectory from every xtc file except for the first one**. In other words, `xtcat` assumes that the first frame of the concatenated trajectory is the same as the last frame of the previous trajectory (but does not check in any way whether this is actually true) and removes it to avoid duplicate frames.

See also the Rust version of this program, [xtcat_rs](https://github.com/Ladme/xtcat_rs).

## Installation

1) Run `make` to create a binary file `xtcat`.
2) (Optional) Run `make install` to copy the the binary file `xtcat` into `${HOME}/.local/bin`.

## Example

```
xtcat -f md0001.xtc md0002.xtc md0003.xtc -o md_cat.xtc
```

The program will concatenate files `md0001.xtc`, `md0002.xtc`, and `md0003.xtc` and write the output into `md_cat.xtc`. The first trajectory frame from `md0002.xtc` and `md0003.xtc` will be removed from the output. The input files will NOT be sorted in any way, neither by their name nor by their starting time. The input files will not be modified.

## Limitations

At most 100 xtc files can be concatenated in a single run.

Only tested on Linux but should work anywhere.
