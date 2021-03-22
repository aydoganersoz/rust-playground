#[derive(Debug)]
pub struct MyStruct {
    field_1: i32,     // private field
    pub field_2: i32, // public field
}

#[derive(Debug, PartialEq)]
pub enum MyPubEnum {
    Field1,
}

#[derive(Debug, PartialEq)]
enum MyPrivEnum {
    Field1,
}

impl MyStruct {
    pub fn pub_fill_default(&mut self) {
        self.field_1 = 0;
        self.field_2 = -1;
    }

    fn fill_default(&mut self) {
        self.field_1 = 0;
        self.field_2 = -2;
    }

    pub fn new() -> MyStruct {
        MyStruct {
            field_1: 0,
            field_2: -3,
        }
    }
}

pub fn func1() -> i32 {
    let mut s = MyStruct {
        field_1: 5,
        field_2: -127,
    };
    assert_eq!(s.field_1, 5);
    assert_eq!(s.field_2, -127);

    s.fill_default(); // private function can be called
    assert_eq!(s.field_1, 0);
    assert_eq!(s.field_2, -2);

    s.pub_fill_default(); // public function can be called
    assert_eq!(s.field_1, 0);
    assert_eq!(s.field_2, -1);

    let pub_enum = MyPubEnum::Field1;
    assert_eq!(pub_enum, MyPubEnum::Field1);

    let priv_enum = MyPrivEnum::Field1;
    assert_eq!(priv_enum, MyPrivEnum::Field1);

    s.field_1 + s.field_2
}
