use ecs2::prelude::*;
use ecs2::query::QueryError;

#[derive(Debug)]
struct MyCmp(usize);

impl Component for MyCmp {}

struct MyUnique(String);

impl Unique for MyUnique {}

#[test]
fn add_and_borrow_comp() {
    let mut world = World::<()>::new();

    let entity = world.spawn().unwrap().insert(MyCmp(12)).unwrap().id();

    let my_cmps = world.borrow::<QueryComp<MyCmp>>().unwrap();
    assert!(matches!(my_cmps.get(entity), Ok(MyCmp(12))));
}

#[test]
fn add_and_borrow_unique() {
    let mut world = World::<()>::new();

    world.insert_unique(MyUnique("hello, world!".to_owned()));

    let mut my_unique_mut = world.borrow::<QueryUniqueMut<MyUnique>>().unwrap();
    my_unique_mut.get_mut().0.push_str(" how are you?");

    assert!(matches!(
        world.borrow::<QueryUnique<MyUnique>>(),
        Err(QueryError::BorrowError(_))
    ));
    drop(my_unique_mut);

    let my_unique = world.borrow::<QueryUnique<MyUnique>>().unwrap();
    assert!(matches!(
        my_unique.get().0.as_str(),
        "hello, world! how are you?"
    ));
}
