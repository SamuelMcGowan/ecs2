use ecs2::prelude::*;
use ecs2::query::QueryResult;
use std::cell::RefMut;

#[derive(Default)]
pub struct GameInfo {
    name: String,
}

impl WorldData for GameInfo {}

pub struct QueryGameInfo;

impl Query<GameInfo> for QueryGameInfo {
    type Output<'a> = RefMut<'a, GameInfo>;

    fn borrow(world: &World<GameInfo>) -> QueryResult<Self::Output<'_>> {
        Ok(world.data.try_borrow_mut()?)
    }
}

#[test]
fn custom_query() {
    let world = World::<GameInfo>::new();

    let mut game_info = world.borrow::<QueryGameInfo>().unwrap();
    game_info.name.push_str("foobar");
    assert_eq!(game_info.name.as_str(), "foobar");
}
