# fern_ndarray_learn
Learn how to use **fern** to write logs in rust, and how to use **ndarray** to read arrays from txt files.



## Files

*Cargo.toml*: we define dependencies including fern and ndarray in this file.

**src/**

*file_util.rs*: contain 2 functions which read 2d array from files.

*lib.rs*: contains a fern logger initializer which set log file name.

**src/bin/**

*data.txt*: example data which contains a 2d array.

*file_load.rs*: test the functions defined in *file_util.rs*.

*output.log*: we can use debug! to write to a common log file.



## How to run

```shell
cd FERN_NDARRAY_LEARN/src/bin
cargo run --bin file_load --release
```

