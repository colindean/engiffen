# engiffen

Generates gifs from image sequences.

![source bitmap](tests/ball/ball01.bmp)
![engiffenned gif](tests/ball.gif)
![photoshopped gif](tests/ball_ps.gif)

_Source frame, generated gif, and a gif from Photoshop_

# usage

## as binary

```bash
# Read a bunch of bitmaps and write them to a 20-frame-per-second gif at path `hello.gif`
engiffen *.bmp -f 20 -o hello.gif

# Read a range of files and write them to `out.gif` (the default output path)
engiffen -r file01.jpg file20.jpg
# The app sorts them in lexicographical order, so if your shell orders `file9`
# before `file10`, the resulting gif will not be in that order.
```

## as library

```rust
extern crate engiffen;

use engiffen::{load_images, engiffen, Gif};
use std::fs::File;

let paths = vec!["vector", "of", "file", "paths", "on", "disk"];
let images = load_images(&paths);
let mut output = File::create("output.gif")?;

// encode an animated gif at 10 frames per second
let gif = engiffen(&images, 10, None)?;
gif.write(&mut output);
```

```rust
// Optionally specify how many pixels of each frame should be sampled
// when computing the gif's palette. This value reduces the amount of
// sampling work to 1/9th of what it normally would, by only sampling
// every 3rd pixel on every 3rd row (i.e. pixels lying on a 3x3 grid).
let gif = engiffen(&images, 10, Some(3));
```

## Major work to do

* Incremental frame processing

  Accept a stream of frames from a server to process individually as they arrive. Put off sorting the final palette and compiling the gif until finished.

## Anything else?

![shrug](tests/shrug.gif)
