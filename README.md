# sketch-csgrs

Learn sketching in csgrs


## Building

Currently requires csgrs source code at "../csgrs".

## Running

NaN's can be passed as "NaN" no quotes on the command line.

```
$ cargo run -- -h
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/sketch-csgrs -h`
sketch-csgrs:+
Create one or more cubes with an optional tube in the center

Usage: sketch-csgrs [OPTIONS] <WIDTH> <HEIGHT>

Arguments:
  <WIDTH>   
  <HEIGHT>  

Options:
  -p, --print-mesh  Enable printing Mesh
  -h, --help        Print help
  -V, --version     Print version
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
