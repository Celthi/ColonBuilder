
# Colon Builder

[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/Celthi/ColonBuilder/rust.yml?branch=main&style=for-the-badge" height="20">](https://github.com/Celthi/ColonBuilder/actions?query=branch%3Amain)

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


# TODO
[ ] support 'require' mark
