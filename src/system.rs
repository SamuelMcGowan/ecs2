use crate::query::{Query, QueryResult};
use crate::world::{World, WorldData};

pub trait System<'a, Data: WorldData, Input, Output> {
    fn run(&self, world: &'a World<Data>) -> QueryResult<Output>;
}

macro_rules! impl_system {
    ($($query:ident),*) => {
        impl<'a, Func, Data, $($query,)* Output>
            System<'a, Data, ($($query,)*), Output>
            for Func
            where
                Func: Fn($($query),*) -> Output,
                Data: WorldData,
                $($query: Query<'a, Data>),*
        {
            #[allow(unused_variables, non_snake_case)]
            fn run(&self, world: &'a World<Data>) -> QueryResult<Output> {
                $(let $query = $query::borrow(world)?;)*
                let output = (self)($($query,)*);
                Ok(output)
            }
        }
    }
}

impl_system!();
impl_system!(Q0);
impl_system!(Q0, Q1);
impl_system!(Q0, Q1, Q2);
impl_system!(Q0, Q1, Q2, Q3);
impl_system!(Q0, Q1, Q2, Q3, Q4);
impl_system!(Q0, Q1, Q2, Q3, Q4, Q5);
impl_system!(Q0, Q1, Q2, Q3, Q4, Q5, Q6);
impl_system!(Q0, Q1, Q2, Q3, Q4, Q5, Q6, Q7);
