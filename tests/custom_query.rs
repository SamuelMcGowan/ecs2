use std::cell::RefMut;

use ecs2::prelude::*;
use ecs2::query::QueryResult;

#[derive(Default)]
pub struct GameInfo {
    name: String,
}

impl WorldData for GameInfo {}

pub struct QueryGameInfo<'a>(RefMut<'a, GameInfo>);

impl<'a> Query<'a, GameInfo> for QueryGameInfo<'a> {
    fn borrow(world: &'a World<GameInfo>) -> QueryResult<Self> {
        Ok(QueryGameInfo(world.data.try_borrow_mut()?))
    }
}

#[test]
fn custom_query() {
    let world = World::<GameInfo>::new();

    let mut game_info = world.borrow::<QueryGameInfo>().unwrap();
    game_info.0.name.push_str("foobar");
    assert_eq!(game_info.0.name.as_str(), "foobar");
}
