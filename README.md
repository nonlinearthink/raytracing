# rst_raytrace

A rust implement for [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html), online version `4.0.0-alpha.1`.

## Run

This project don't provide a bin target, so you can't just run:

```bash
cargo run
cargo run --release
```

But there are some example targets, to get a list of available examples, run a command that doesn't specify any examples.

```bash
cargo run --release --example
# You will get a output like this:
#
# Available examples:
#    dev_scene_1
#    final_scene_1
```

Pick and run a specify example:

```bash
cargo run --release --example final_scene_1
```

>`--release` is recommended, because it really speeds up the render process.

## Overview

### Ray Tracing In One Weekend

![Ray Tracing In One Weekend](images/first-book-final-scene.jpg)

### Ray Tracing The Next Week

todo...

### Ray Tracing The Rest of Your Life

todo...
