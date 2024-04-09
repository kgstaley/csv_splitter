# simple CSV Splitter written in Rust 

run it like: 
```rust
cargo run -- path/to/directory/file.csv <num_pieces>

# e.g. 
cargo run -- ~/Downloads/some_csv.csv 4
```

be default, I kept it simple and it will just write the split .csv files to the root directory

## TODO
- [ ] allow optional arg for passing which directory to write files to
- [ ] add tests
