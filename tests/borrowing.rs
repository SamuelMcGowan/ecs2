use ecs2::prelude::*;

#[derive(Debug)]
struct MyCmp(usize);
impl Component for MyCmp {}

#[test]
fn add_and_borrow() {
    let mut world = World::new();

    let entity = world.spawn().unwrap();

    let mut my_cmps_mut = world.borrow::<CompMut<MyCmp>>().unwrap();
    my_cmps_mut.insert(entity, MyCmp(12)).unwrap();

    assert!(matches!(my_cmps_mut.get(entity), Ok(MyCmp(12))));

    assert!(matches!(world.borrow::<Comp<MyCmp>>(), Err(QueryError::BorrowError(_))));
    drop(my_cmps_mut);

    let my_cmps = world.borrow::<Comp<MyCmp>>().unwrap();
    assert!(matches!(my_cmps.get(entity), Ok(MyCmp(12))));
}
