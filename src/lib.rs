use mytrait_derive::MyTrait;

trait MyTrait {
    fn answer() -> i32 {
        42
    }
}

#[derive(MyTrait)]
struct Foo;

// impl MyTrait for Foo{

// }

#[test]
fn default() {
    assert_eq!(Foo::answer(), 42);
}

