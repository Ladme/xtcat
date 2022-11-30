# xtcat_rs: Rust version of xtcat

`xtcat_rs` is a Rust version of the C version [xtcat](https://github.com/Ladme/xtcat). Little safer with less limitations than the C version.

It is to be used **only** for concatenating xtc files that come from simulation runs directly following each other. `xtcat_rs` is actually just a slightly smarter `cat` as it stitches all xtc files together while **removing the first frame of the trajectory from every xtc file except for the first one**. In other words, `xtcat` assumes that the first frame of the concatenated trajectory is the same as the last frame of the previous trajectory (but does not check in any way whether this is actually true) and removes it to avoid duplicate frames.

## Installation

0) Install [rust](https://www.rust-lang.org/tools/install).
1) Run `cargo build --release`.
2) Use the binary file `xtcat_rs` generated in `target/release/` anywhere.


## Example usage

```
xtcat_rs -f md0001.xtc md0002.xtc md0003.xtc -o md_cat.xtc
```

The program will concatenate files `md0001.xtc`, `md0002.xtc`, and `md0003.xtc` and write the output into `md_cat.xtc`. The first trajectory frame from `md0002.xtc` and `md0003.xtc` will be removed from the output. The input files will NOT be sorted in any way, neither by their name nor by their starting time. The input files will not be modified.

In case `-o` is not supplied, default option of `output.xtc` will be used.

## Limitations

Only tested on Linux but should work anywhere.