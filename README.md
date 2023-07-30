# RayTracer_Rust_Implementation
An implementation of https://raytracing.github.io/books/RayTracingInOneWeekend.html but in Rust instead of C++. This is done in an attempt to learn Rust for the first time. Please do not copy the code as there are probably many badly written lines of code

## Run
To run this project use `cargo run > ImageName.ppm`. This will generate a ppm file with the render.

## Change settings
In `main.rs` in the function `main` you can change the following settings:
```// Settings
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 1920;
const SAMPLES_PER_PIXEL: usize = 30;
const MAX_DEPTH: usize = 5;```