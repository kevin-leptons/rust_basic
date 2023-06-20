use rust_basic::HashMap;
use testkit::NonZeroSize;

pub(super) fn sample<'a>() -> HashMap<NonZeroSize, NonZeroSize<(&'a str, usize)>>
{
    let mut map = HashMap::new();
    for i in 0..10000 {
        let key = NonZeroSize::new(i);
        let value = NonZeroSize::new(("value", i));
        map.set(key, value);
    }
    return map;
}
