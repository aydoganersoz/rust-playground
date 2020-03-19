#[derive(Debug)]
pub struct MyStruct {
    field_1: u32,    // private field
    pub field_2: i8, // public field
}

#[derive(Debug)]
pub enum MyPubEnum {
    Field1,
}

#[derive(Debug)]
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

pub fn func1() {
    let mut s = MyStruct {
        field_1: 5,
        field_2: -127,
    };
    println!("\tI am func1 in my_module_3");
    println!("\t{:?}", s);
    s.fill_default(); // private function can be called
    println!("\t{:?}", s);
    s.pub_fill_default(); // public function can be called
    println!("\t{:?}", s);
    let pub_enum = MyPubEnum::Field1;
    println!("\t{:?}", pub_enum);
    let priv_enum = MyPrivEnum::Field1;
    println!("\t{:?}", priv_enum);
}
