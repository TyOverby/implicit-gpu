type PathSegment = ::PathSegment<::euclid::UnknownUnit>;

pub fn rotate_1<T>(arr: &mut [T]) {
    {
        let (left, right) = arr.split_at_mut(1);
        left.reverse();
        right.reverse();
    }
    arr.reverse();
}

pub fn assert_same(
    actual: &[PathSegment],
    expected: &[PathSegment],
    permit_reversed: bool,
) -> Result<(), String> {
    if actual.len() != expected.len() {
        return Err(format!(
            "assert_same wrong lengths {} vs {}\n actual: {:?}\n expected: {:?}",
            actual.len(),
            expected.len(),
            actual,
            expected
        ));
    }

    for ex in expected {
        let mut found = false;
        for ac in actual {
            if is_equal(&ex, &ac, permit_reversed) {
                found = true;
                break;
            }
        }

        if !found {
            return Err(format!(
                "{:?} not found in actual\n actual: {:?}\n expected: {:?}",
                ex,
                actual,
                expected
            ));
        }
    }

    return Ok(());
}

pub fn is_equal(expected: &PathSegment, actual: &PathSegment, permit_reversed: bool) -> bool {
    if expected.path.len() != actual.path.len() || expected.closed != actual.closed {
        return false;
    }
    let mut path = actual.path.clone();
    let basic_shifted = is_shifted_of(&expected.path, &mut path);
    let reverse_shifted = permit_reversed && {
        path.reverse();
        is_shifted_of(&expected.path, &mut path)
    };

    return basic_shifted || reverse_shifted;
}

pub fn is_shifted_of<T: PartialEq + ::std::fmt::Debug>(goal: &[T], actual: &mut [T]) -> bool {
    if goal.len() != actual.len() {
        return false;
    }
    for _ in 0..goal.len() {
        if goal == actual {
            return true;
        }
        rotate_1(actual);
    }

    return false;
}
