# About 'semantic_triples'

This project is an example project used in my video tutorial on [YouTube](https://youtu.be/TJMwMqD4XSo).

Its aim is to show how to parse a list of semantic triples in the form
```
Subject --predicate--> Object
```
with the help of the parser generator [parol](https://crates.io/crates/parol) and transform the
parsed information into Graphviz' dot file format.


To run the parser against the test data you can call

```shell
cargo run ./test.txt
```

in the crate's root folder.

## License

This project is free, open source and permissively licensed! Except where noted (below and/or in
individual files), all code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or
[http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
[http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
This means you can select the license you prefer!
