
# Colon Builder

[![Build status](https://github.com/Celthi/ColonBuilder/actions/workflows/rust.yml/badge.svg)](https://github.com/Celthi/ColonBuilder/blob/main/.github/workflows/rust.yml)

Build structure from colon separate fields like
```
name:colon builder
hobbies: book,game,football
TV: once a while
```
Sample code
```rust
#[derive(ColonBuilder)]
struct Person {
    #[cb(require)]
    name: String,
    hobbies: Vec<String>,
    #[cb(abbr="TV")]
    favoriteTVShows: Option<String>
}

let p = Person::from_str("name:lu\nTV:show");

```


`cargo expand --test test_builder ` to expand the test file `test_builder.rs`


