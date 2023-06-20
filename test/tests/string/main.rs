use rust_basic::string::String;

#[test]
fn new() {
    let string = String::new();
    assert_eq!(string.size(), 0);
}

#[test]
fn from() {
    let raw = "foo bar baz";
    let string = String::from(raw);
    assert_eq!(string.as_str(), raw);
    assert_eq!(string.size(), raw.len());
}

#[test]
fn from_empty() {
    let string = String::from("");
    assert_eq!(string.as_str(), "");
    assert_eq!(string.size(), 0);
}

#[test]
fn equal_true() {
    let string0 = String::from("one two three");
    let string1 = String::from("one two three");
    assert_eq!(string0, string1);
}

#[test]
fn equal_false() {
    let string0 = String::from("one two three");
    let string1 = String::from("one two four");
    assert_ne!(string0, string1);
}

#[test]
fn order_greater_size_equal() {
    let string0 = String::from("30");
    let string1 = String::from("20");
    assert!(string0 > string1);
}

#[test]
fn order_greater_size_shoter() {
    let string0 = String::from("3");
    let string1 = String::from("20");
    assert!(string0 > string1);
}

#[test]
fn order_greater_size_longer() {
    let string0 = String::from("200");
    let string1 = String::from("20");
    assert!(string0 > string1);
}

#[test]
fn slice_head() {
    let string0 = String::from("one two three");
    let string1 = string0.slice(0, 3);
    assert_eq!(string1, String::from("one"));
}

#[test]
fn slice_middle() {
    let string0 = String::from("one two three");
    let string1 = string0.slice(4, 7);
    assert_eq!(string1, String::from("two"));
}

#[test]
fn slice_tail() {
    let string0 = String::from("one two three");
    let string1 = string0.slice(8, 13);
    assert_eq!(string1, String::from("three"));
}

#[test]
fn slice_head_from_equal_to() {
    let string0 = String::from("one two three");
    let string1 = string0.slice(0, 0);
    assert_eq!(string1, String::from(""));
}

#[test]
fn slice_middle_from_equal_to() {
    let string0 = String::from("one two three");
    let string1 = string0.slice(3, 3);
    assert_eq!(string1, String::from(""));
}

#[test]
fn slice_end_from_equal_to() {
    let string0 = String::from("one two three");
    let string1 = string0.slice(12, 12);
    assert_eq!(string1, String::from(""));
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn slice_panic_from_greater_than_to() {
    let string = String::from("one two three");
    string.slice(2, 1);
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn slice_panic_to_equal_to_size() {
    let string = String::from("one two three");
    string.slice(1, 14);
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn slice_panic_to_greater_than_size() {
    let string = String::from("one two three");
    string.slice(1, 15);
}

#[test]
fn insert_head() {
    let mut string0 = String::from("one three");
    let string1 = String::from("two ");
    string0.insert(0, &string1);
    assert_eq!(string0, String::from("two one three"));
}

#[test]
fn insert_middle() {
    let mut string0 = String::from("one three");
    let string1 = String::from("two ");
    string0.insert(4, &string1);
    assert_eq!(string0, String::from("one two three"));
}

#[test]
fn insert_tail() {
    let mut string0 = String::from("one three");
    let string1 = String::from(" two");
    string0.insert(9, &string1);
    assert_eq!(string0, String::from("one three two"));
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn insert_panic() {
    let mut string0 = String::from("one three");
    let string1 = String::from(" two");
    string0.insert(10, &string1);
}

#[test]
fn insert_empty() {
    let mut string0 = String::from("one three");
    let string1 = String::from("");
    string0.insert(9, &string1);
    assert_eq!(string0, String::from("one three"));
}

#[test]
fn replace_not_found() {
    let mut string = String::from("one two three");
    let pattern = String::from("four");
    let value = String::from("five");
    string.replace(&pattern, &value);
    assert_eq!(string, String::from("one two three"));
}

#[test]
fn replace_empty_value() {
    let mut string = String::from("two one two three two six two");
    let pattern = String::from("two");
    let value = String::from("");
    string.replace(&pattern, &value);
    assert_eq!(string, String::from(" one  three  six "));
}

#[test]
fn replace_size_narrow() {
    let mut string = String::from("one three two three six");
    let pattern = String::from("three");
    let value = String::from("ten");
    string.replace(&pattern, &value);
    assert_eq!(string, String::from("one ten two ten six"));
}

#[test]
fn replace_size_narrow_head_tail() {
    let mut string = String::from("eight two three two eight");
    let pattern = String::from("eight");
    let value = String::from("one");
    string.replace(&pattern, &value);
    assert_eq!(string, String::from("one two three two one"));
}

#[test]
fn replace_size_expand() {
    let mut string = String::from("one two three two ten");
    let pattern = String::from("two");
    let value = String::from("eight");
    string.replace(&pattern, &value);
    assert_eq!(string, String::from("one eight three eight ten"));
}

#[test]
fn replace_size_expand_head_tail() {
    let mut string = String::from("one two three two one");
    let pattern = String::from("one");
    let value = String::from("three");
    string.replace(&pattern, &value);
    assert_eq!(string, String::from("three two three two three"));
}

#[test]
fn replace_size_unchange() {
    let mut string = String::from("one two three two four");
    let pattern = String::from("two");
    let value = String::from("ten");
    string.replace(&pattern, &value);
    assert_eq!(string, String::from("one ten three ten four"));
}

#[test]
fn replace_size_unchange_head_tail() {
    let mut string = String::from("one two three two one");
    let pattern = String::from("one");
    let value = String::from("six");
    string.replace(&pattern, &value);
    assert_eq!(string, String::from("six two three two six"));
}

#[test]
#[should_panic(expected = "expect: not empty pattern")]
fn replace_panic_empty_pattern() {
    let mut string = String::from("one two three");
    let pattern = String::from("");
    let value = String::from("four give");
    string.replace(&pattern, &value);
}

#[test]
fn append() {
    let mut string0 = String::from("one two");
    let string1 = String::from(" three");
    string0.append(&string1);
    assert_eq!(string0, String::from("one two three"));
}

#[test]
fn trim() {
    let mut string = String::from("  \t  one  two  \t  ");
    string.trim();
    assert_eq!(string, String::from("one  two"));
}

#[test]
fn trim_all_whitespaces() {
    let mut string = String::from("  \t   \t  \t  ");
    string.trim();
    assert_eq!(string, String::from(""));
}

#[test]
fn trim_head_single() {
    let mut string = String::from(" one  two");
    string.trim();
    assert_eq!(string, String::from("one  two"));
}

#[test]
fn trim_head_multi() {
    let mut string = String::from(" \t one  two");
    string.trim();
    assert_eq!(string, String::from("one  two"));
}

#[test]
fn trim_tail_single() {
    let mut string = String::from("one two ");
    string.trim();
    assert_eq!(string, String::from("one two"));
}

#[test]
fn trim_tail_multi() {
    let mut string = String::from("one two \t ");
    string.trim();
    assert_eq!(string, String::from("one two"));
}

#[test]
fn upper() {
    let mut string = String::from(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz 0123456789",
    );
    string.upper();
    assert_eq!(
        string,
        String::from(
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ ABCDEFGHIJKLMNOPQRSTUVWXYZ 0123456789"
        )
    );
}

#[test]
fn lower() {
    let mut string = String::from(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz 0123456789",
    );
    string.lower();
    assert_eq!(
        string,
        String::from(
            "abcdefghijklmnopqrstuvwxyz abcdefghijklmnopqrstuvwxyz 0123456789"
        )
    );
}

#[test]
fn get() {
    let string = String::from("a b C");
    assert_eq!(string.get(0), 'a');
    assert_eq!(string.get(1), ' ');
    assert_eq!(string.get(2), 'b');
    assert_eq!(string.get(4), 'C');
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn get_panic() {
    let string = String::from("0123");
    string.get(4);
}

#[test]
fn get_code() {
    let string = String::from("a b C");
    assert_eq!(string.get_code(0), 0x61);
    assert_eq!(string.get_code(1), 0x20);
    assert_eq!(string.get_code(2), 0x62);
    assert_eq!(string.get_code(4), 0x43);
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn get_code_panic() {
    let string = String::from("a b C");
    string.get_code(5);
}

#[test]
fn find() {
    let string = String::from("aaa aaab xyz aaaa ab aaa ba aaaa");
    let pattern = String::from("aaaa");
    let mut iter = string.find(&pattern);
    assert_eq!(iter.next(), Some(13));
    assert_eq!(iter.next(), Some(28));
    assert_eq!(iter.next(), None);
}

#[test]
fn find_not_found() {
    let string = String::from("aaa aaab xyz aaaa ab aaa ba aaaa");
    let pattern = String::from("aba");
    let mut iter = string.find(&pattern);
    assert_eq!(iter.next(), None);
}

#[test]
fn find_not_found_pattern_size() {
    let string = String::from("aaa aaab xyz aaaa ab aaa ba aaaa");
    let pattern = String::from("aaa aaab xyz aaaa ab aaa ba aaaa ab");
    let mut iter = string.find(&pattern);
    assert_eq!(iter.next(), None);
}

#[test]
fn find_empty() {
    let string = String::from("");
    let pattern = String::from("bb");
    let mut iter = string.find(&pattern);
    assert_eq!(iter.next(), None);
}

#[test]
#[should_panic(expected = "expect: not empty pattern")]
fn find_panic_empty_pattern() {
    let string = String::from("aaa aab");
    let pattern = String::from("");
    string.find(&pattern);
}

#[test]
fn clear() {
    let mut string = String::from("aaa aaab xyz aaaa ab aaa ba aaaa");
    string.clear();
    assert_eq!(string.size(), 0);
}

#[test]
fn clone() {
    let string0 = String::from("foo bar baz");
    let string1 = string0.clone();
    assert_eq!(string0, string1);
}

// This test does nothing but creating a non empty container to trigger memory
// release process. The test can not work alone, it requries an external tool
// such as Valgrind to diagnose memory issues.
//
// Warn: The test maybe still passed even memory release process has issues.
#[test]
fn drop() {
    let _ = String::from("foo bar baz");
}
