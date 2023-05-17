use rust_basic::string::String;
use rust_basic::vector::Vector;

#[test]
fn from() {
    for i in 0..1000 {
        let a = format!("this is a string: {}", i);
        let b = a.as_str();
        let s = String::from(b);
        assert_eq!(s.as_str(), b);
    }
}

#[test]
fn size() {
    let a = "this is a string";
    let s = String::from(a);
    assert_eq!(s.size(), 16);
}

#[test]
fn equal() {
    let s0 = String::from("one two three");
    let s1 = String::from("one two three");
    assert_eq!(s0, s1);
}

#[test]
fn equal_no() {
    let s0 = String::from("one two three");
    let s1 = String::from("one two four");
    assert_ne!(s0, s1);
}

#[test]
fn greater_same_size() {
    let s0 = String::from("30");
    let s1 = String::from("20");
    assert!(s0 > s1);
}

#[test]
fn greater_shoter() {
    let s0 = String::from("3");
    let s1 = String::from("20");
    assert!(s0 > s1);
}

#[test]
fn greater_longer() {
    let s0 = String::from("200");
    let s1 = String::from("20");
    assert!(s0 > s1);
}

#[test]
fn slice_head() {
    let s1 = String::from("one two three");
    let s2 = s1.slice(0, 3);
    assert_eq!(s2.as_str(), "one");
}

#[test]
fn slice_middle() {
    let s1 = String::from("one two three");
    let s2 = s1.slice(4, 7);
    assert_eq!(s2.as_str(), "two");
}

#[test]
#[should_panic(expected = "expect: `from` is less than `to`")]
fn slice_panic_from_equal_to() {
    let s1 = String::from("one two three");
    let s2 = s1.slice(1, 1);
}

#[test]
#[should_panic(expected = "expect: `to` is not greater than size")]
fn slice_panic_greater_than_size() {
    let s1 = String::from("one two three");
    let s2 = s1.slice(1, 14);
}

#[test]
#[should_panic(expected = "expect: `from` is less than `to`")]
fn slice_panic_from_greater_than_to() {
    let s1 = String::from("one two three");
    let s2 = s1.slice(2, 1);
}

#[test]
fn slice_tail() {
    let s1 = String::from("one two three");
    let s2 = s1.slice(8, 13);
    assert_eq!(s2.as_str(), "three");
}

#[test]
fn insert_head() {
    let mut s1 = String::from("one three");
    let s2 = String::from("two ");
    s1.insert(0, &s2);
    assert_eq!(s1.as_str(), "two one three");
}

#[test]
fn insert_middle() {
    let mut s1 = String::from("one three");
    let s2 = String::from("two ");
    s1.insert(4, &s2);
    assert_eq!(s1.as_str(), "one two three");
}

#[test]
fn insert_tail() {
    let mut s1 = String::from("one three");
    let s2 = String::from(" two");
    s1.insert(9, &s2);
    assert_eq!(s1.as_str(), "one three two");
}

#[test]
#[should_panic(expected = "expect: `to` is not greater than size")]
fn insert_panic() {
    let mut s1 = String::from("one three");
    let s2 = String::from(" two");
    s1.insert(10, &s2);
}

#[test]
fn replace_unchange() {
    let mut s1 = String::from("one two three");
    let pattern = String::from("four");
    let value = String::from("five");
    s1.replace(&pattern, &value);
    assert_eq!(s1.as_str(), "one two three");
    assert_eq!(s1.size(), 13);
}

#[test]
fn replace_empty_value() {
    let mut s1 = String::from("two one two three two six two");
    let pattern = String::from("two");
    let value = String::from("");
    s1.replace(&pattern, &value);
    assert_eq!(s1.as_str(), " one  three  six ");
    assert_eq!(s1.size(), 17);
}

#[test]
fn replace_size_narrow() {
    let mut s1 = String::from("one three two three six");
    let pattern = String::from("three");
    let value = String::from("ten");
    s1.replace(&pattern, &value);
    assert_eq!(s1.as_str(), "one ten two ten six");
    assert_eq!(s1.size(), 19);
}

#[test]
fn replace_size_narrow_head_tail() {
    let mut s1 = String::from("eight two three two eight");
    let pattern = String::from("eight");
    let value = String::from("one");
    s1.replace(&pattern, &value);
    assert_eq!(s1.as_str(), "one two three two one");
    assert_eq!(s1.size(), 21);
}

#[test]
fn replace_size_expand() {
    let mut s1 = String::from("one two three two ten");
    let pattern = String::from("two");
    let value = String::from("eight");
    s1.replace(&pattern, &value);
    assert_eq!(s1.as_str(), "one eight three eight ten");
    assert_eq!(s1.size(), 25);
}

#[test]
fn replace_size_expand_head_tail() {
    let mut s1 = String::from("one two three two one");
    let pattern = String::from("one");
    let value = String::from("three");
    s1.replace(&pattern, &value);
    assert_eq!(s1.as_str(), "three two three two three");
    assert_eq!(s1.size(), 25);
}

#[test]
fn replace_size_unchange() {
    let mut s1 = String::from("one two three two four");
    let pattern = String::from("two");
    let value = String::from("ten");
    s1.replace(&pattern, &value);
    assert_eq!(s1.as_str(), "one ten three ten four");
    assert_eq!(s1.size(), 22);
}

#[test]
fn replace_size_unchange_head_tail() {
    let mut s1 = String::from("one two three two one");
    let pattern = String::from("one");
    let value = String::from("six");
    s1.replace(&pattern, &value);
    assert_eq!(s1.as_str(), "six two three two six");
    assert_eq!(s1.size(), 21);
}

#[test]
#[should_panic(expected = "expect: `pattern` is not empty")]
fn replace_panic_pattern() {
    let mut s1 = String::from("one two three");
    let pattern = String::from("");
    let value = String::from("four give");
    s1.replace(&pattern, &value);
}

#[test]
fn append() {
    let mut s1 = String::from("one two");
    let s2 = String::from(" three");
    s1.append(&s2);
    assert_eq!(s1.as_str(), "one two three");
}

#[test]
fn trim() {
    let mut s1 = String::from("  \t  one  two  \t  ");
    s1.trim();
    assert_eq!(s1.as_str(), "one  two");
}

#[test]
fn trim_head_single() {
    let mut s1 = String::from(" one  two");
    s1.trim();
    assert_eq!(s1.as_str(), "one  two");
}

#[test]
fn trim_head_multi() {
    let mut s1 = String::from(" \t one  two");
    s1.trim();
    assert_eq!(s1.as_str(), "one  two");
}

#[test]
fn trim_tail_single() {
    let mut s1 = String::from("one two ");
    s1.trim();
    assert_eq!(s1.as_str(), "one two");
}

#[test]
fn trim_tail_multi() {
    let mut s1 = String::from("one two \t ");
    s1.trim();
    assert_eq!(s1.as_str(), "one two");
}

#[test]
fn upper() {
    let mut s1 = String::from(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz 0123456789",
    );
    s1.upper();
    assert_eq!(
        s1.as_str(),
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ ABCDEFGHIJKLMNOPQRSTUVWXYZ 0123456789"
    );
}

#[test]
fn lower() {
    let mut s1 = String::from(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz 0123456789",
    );
    s1.lower();
    assert_eq!(
        s1.as_str(),
        "abcdefghijklmnopqrstuvwxyz abcdefghijklmnopqrstuvwxyz 0123456789"
    );
}

#[test]
fn code_at() {
    let s = String::from("a b C");
    assert_eq!(s.code_at(0), 0x61);
    assert_eq!(s.code_at(2), 0x62);
    assert_eq!(s.code_at(4), 0x43);
}

#[test]
#[should_panic(expected = "expect: `index` is less than size")]
fn code_at_panic() {
    let s = String::from("a b C");
    s.code_at(5);
}

#[test]
#[should_panic(expected = "expect: `index` is less than size")]
fn char_at() {
    let s = String::from("a b C");
    s.char_at(5);
}

#[test]
fn find() {
    let s = String::from("aaa aaab xyz aaaa ab aaa ba aaaa");
    let p = String::from("aaaa");
    let mut i = s.find(&p);
    assert_eq!(i.next(), Some(13));
    assert_eq!(i.next(), Some(28));
    assert_eq!(i.next(), None);
}

#[test]
fn find_not_found() {
    let s = String::from("aaa aaab xyz aaaa ab aaa ba aaaa");
    let p = String::from("aba");
    let mut i = s.find(&p);
    assert_eq!(i.next(), None);
}

#[test]
fn find_pattern_too_long() {
    let s = String::from("aaa aaab xyz aaaa ab aaa ba aaaa");
    let p = String::from("aaa aaab xyz aaaa ab aaa ba aaaa ab");
    let mut i = s.find(&p);
    assert_eq!(i.next(), None);
}

#[test]
fn find_empty_string() {
    let s = String::from("");
    let p = String::from("bb");
    let mut i = s.find(&p);
    assert_eq!(i.next(), None);
}

#[test]
#[should_panic(expected = "expect: `pattern` is not empty")]
fn find_empty_pattern() {
    let s = String::from("aaa aab");
    let p = String::from("");
    s.find(&p);
}

#[test]
#[should_panic(expected = "expect: `pattern` is not empty")]
fn find_empty_string_and_pattern() {
    let s = String::from("");
    let p = String::from("");
    let mut i = s.find(&p);
    assert_eq!(i.next(), None);
}

#[test]
fn clear() {
    let mut s = String::from("aaa aaab xyz aaaa ab aaa ba aaaa");
    assert!(s.size() > 0);
    s.clear();
    assert_eq!(s.size(), 0);
}
