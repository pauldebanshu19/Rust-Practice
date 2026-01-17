use generic_type::*;

#[test]
fn test_first() {
    assert_eq!(first(('a', false)), 'a');
    assert_eq!(first((1u32, 0i32)), 1u32);
}

#[test]
fn test_last() {
    assert_eq!(last(('a', "hello")), "hello");
    assert_eq!(last((1u32, 0i32)), 0i32);
}

#[test]
fn test_rectangle() {
    let r_u: Rectangle<u32> = Rectangle {
        top: 0,
        left: 0,
        width: 100,
        height: 100,
    };

    let r_f: Rectangle<f32> = Rectangle {
        top: 0.0,
        left: 0.0,
        width: 100.05,
        height: 100.05,
    };

    assert_eq!(r_u.width, 100);
    assert_eq!(r_f.width, 100.05);
}
