struct Bar<'a> {
    foo: &'a i32,
}

fn test1() {
    let foo_a = 20;
    let foo_b = 23;

    let a = create_bar(&foo_a);
    let b = create_bar(&foo_b);

    assert_eq!(get_smaller_foo(&a, &b), &20);
}

fn create_bar(arg: &i32) -> Bar {
    Bar { foo: arg }
}

fn get_smaller_foo<'a>(a: &'a Bar, b: &'a Bar) -> &'a i32 {
    if a.foo < b.foo {
        a.foo
    } else {
        b.foo
    }
}

pub fn test() {
    test1();
}
