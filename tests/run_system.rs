use ecs2::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct Foo(usize);
impl Component for Foo {}

#[test]
fn run_system() {
    let mut world = World::<()>::new();

    let a = world.spawn().unwrap();

    world
        .run(|mut query: QueryCompMut<Foo>| {
            query.insert(a, Foo(12)).unwrap();
            assert_eq!(query.get(a).unwrap(), &Foo(12));
        })
        .unwrap();
}

#[test]
fn query_conflict() {
    let world = World::<()>::new();
    world
        .run(|_q1: QueryCompMut<Foo>, _q2: QueryCompMut<Foo>| {})
        .unwrap_err();
}
