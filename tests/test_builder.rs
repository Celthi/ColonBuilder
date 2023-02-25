// #[macro_use]
// extern crate colonbuilder;

use colonbuilder::ColonBuilder;
use regex;

macro_rules! reg {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

#[allow(non_snake_case)]
#[derive(ColonBuilder)]
struct Person {
    #[cb(require)]
    name: String,
    hobbies: Vec<String>,
    #[cb(abbr="TV")]
    favoriteTVShows: Option<String>
}

#[test]
fn test_abbr() {
    let p = Person::from_str("name:lu\nTV:show");
    assert_eq!(p.name, "lu");
    assert_eq!(p.hobbies, Vec::<String>::new());
    assert_eq!(p.favoriteTVShows, Some("show".to_string()));
    let p = Person::from_str("name:lu\n");
    assert_eq!(p.name, "lu");
    assert_eq!(p.hobbies, Vec::<String>::new());
    assert_eq!(p.favoriteTVShows, None);
}
