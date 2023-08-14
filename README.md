# macro-master

> Conquer the challenge, be a macro master


## Install

```shell
git clone https://github.com/rust-redo/macro-master.git
```

## Challenges

> Tip: for a better understanding of each macro, it's recommended to read comments and test cases first.
> For procedural macros, you can use `quote` and `syn` crates to simplify the `TokenStream` process.

Running a challenge is simple, go to the challenge module and implement macro to make all tests pass. You can run `cargo test [test_name]` for a specific macro's test cases.


### macro_rules

- [`arg!`](./macro_rules/src/arg.rs)

### procedural macro

- [`HeapSize`](./proc_macros/src/heap_size.rs)

## References

All the challenges come from simplified implementations of these excellent projects below.

- [quote](https://github.com/dtolnay/quote)
- [syn](https://github.com/dtolnay/syn)
- [clap](https://github.com/clap-rs/clap)