use crate::Vector;

pub(super) fn sort<T>(vector: &mut Vector<T>)
where
    T: Ord,
{
    if vector.size <= 1 {
        return;
    }
    let mut vector1 = Vector::<T>::new();
    let mut vector2 = Vector::<T>::new();
    let border = vector.size / 2;
    for _ in 0..border {
        vector1.push_back(vector.pop_back());
    }
    for _ in 0..vector.size {
        vector2.push_back(vector.pop_back());
    }
    sort(&mut vector1);
    sort(&mut vector2);
    merge(&mut vector1, &mut vector2, vector);
}

fn merge<T>(
    sorted1: &mut Vector<T>,
    sorted2: &mut Vector<T>,
    target: &mut Vector<T>,
) where
    T: Ord,
{
    loop {
        if sorted1.size() == 0 || sorted2.size() == 0 {
            break;
        }
        if sorted1[0] <= sorted2[0] {
            target.push_back(sorted1.pop_front());
        } else {
            target.push_back(sorted2.pop_front());
        }
    }
    for _ in 0..sorted1.size() {
        target.push_back(sorted1.pop_front());
    }
    for _ in 0..sorted2.size() {
        target.push_back(sorted2.pop_front());
    }
}
