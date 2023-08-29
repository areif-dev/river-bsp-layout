# river-bsp-layout

Custom River layout manager that creates a Binary Space Partition / Grid layout using river-layout-toolkit

3 Window Example:
```
+----------------+----------------+
|                |                |
|                |                |
|                +----------------+
|                |                |
|                |                |
+----------------+----------------+
```

4 Window Example:
```
+----------------+----------------+
|                |                |
|                |                |
+----------------+----------------+
|                |                |
|                |                |
+----------------+----------------+
```

## Installing 

```bash
cargo install river-bsp-layout --locked
```

## Building

* Make sure you have `cargo` installed. The recommended means of installation is with `rustup.sh` from https://www.rust-lang.org/tools/install
* Clone this repo with `git clone https://github.com/areif-dev/river-bsp-layout`
* Enter the cloned directory `cd river-bsp-layout`
* run `cargo build --release`
* Move the compiled binary from `target/release/river-bsp-layout` to any directory in your `$PATH`
  * With `cargo` installed, one option would be to move the the binary to `$HOME/.cargo/bin`

## CLI

`river-bsp-layout` currently accepts two options:
* `--inner-gap` or `-i`: Number of pixels to place between each adjacent container
  * Defaults to 5
* `--outer-gap` or `-o`: Number of pixels to place between the edge of the display and the edge of each container
  * Defaults to 5
 
## riverctl Commands

`river-bsp-layout` supports two commands sent from `riverctl send-layout-cmd`:
* `riverctl send-layout-cmd bsp-layout "outer-gap #"` will change the outer gap to `#` if `#` is a positive integer
* `riverctl send-layout-cmd bsp-layout "inner-gap #"` will change the inner gap to `#` if `#` is a positive integer

## Starting from River init

Assuming you are using a default `bash` init script, replace the following lines at the end of the file

```bash
riverctl default-layout rivertile
rivertile -view-padding 6 -outer-padding 6 &
```

with 

```bash
riverctl default-layout bsp-layout
river-bsp-layout --inner-gap 10 --outer-gap 10 &
```
