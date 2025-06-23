# Command-Line Help for `balasheet`

This document contains the help content for the `balasheet` command-line program.

**Command Overview:**

* [`balasheet`↴](#balasheet)
* [`balasheet round-corners`↴](#balasheet-round-corners)
* [`balasheet resize`↴](#balasheet-resize)
* [`balasheet new`↴](#balasheet-new)

## `balasheet`

**Usage:** `balasheet <COMMAND>`

###### **Subcommands:**

* `round-corners` — Round the corners of each tile in a spritesheet
* `resize` — Resize an image to the desired scale
* `new` — Create a new spritesheet



## `balasheet round-corners`

Round the corners of each tile in a spritesheet

**Usage:** `balasheet round-corners [OPTIONS] <INPUT_IMAGE> [OUTPUT_IMAGE]`

###### **Arguments:**

* `<INPUT_IMAGE>` — Path to the input image
* `<OUTPUT_IMAGE>` — Path to the output image

###### **Options:**

* `-w`, `--width <WIDTH>` — Width of each tile in pixels

  Default value: `144`
* `-h`, `--height <HEIGHT>` — Height of each tile in pixels

  Default value: `190`
* `-r`, `--radius <RADIUS>` — Corner radius in pixels

  Default value: `10`



## `balasheet resize`

Resize an image to the desired scale

**Usage:** `balasheet resize --scale <SCALE_FACTOR> <INPUT_IMAGE> <OUTPUT_IMAGE>`

###### **Arguments:**

* `<INPUT_IMAGE>` — The path to the input image
* `<OUTPUT_IMAGE>` — The path to the resulting image

###### **Options:**

* `-s`, `--scale <SCALE_FACTOR>` — The factor to scale the given spritesheet to



## `balasheet new`

Create a new spritesheet

**Usage:** `balasheet new [OPTIONS] --columns <COLUMNS> --rows <ROWS> <PATH>`

###### **Arguments:**

* `<PATH>` — The path to the new spritesheet

###### **Options:**

* `-w`, `--tile-width <TILE_WIDTH>` — Width of each tile in pixels

  Default value: `144`
* `-h`, `--tile-height <TILE_HEIGHT>` — Height of each tile in pixels

  Default value: `190`
* `-c`, `--columns <COLUMNS>` — The total amount of columns in the new spritesheet
* `-r`, `--rows <ROWS>` — The total amount of rows in the new spritesheet



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
