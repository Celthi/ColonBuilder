
# Colon Builder
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
```


`cargo expand --test p ` to expand the test file `p.rs`
