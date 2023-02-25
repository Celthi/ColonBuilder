
# Colon Builder
[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/proc--macro2-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/Celthi/ColonBuilder.git)
[<img alt="crates.io" src="https://img.shields.io/crates/v/proc-macro2.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/ColonBuilder)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-proc--macro2-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/ColonBuilder)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/dtolnay/proc-macro2/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/Celthi/ColonBuilder.git/actions?query=branch%3Amaster)
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
