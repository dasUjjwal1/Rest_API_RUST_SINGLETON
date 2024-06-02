use async_graphql::MergedObject;
use task::task_resolver::TaskResolver;
use user::{user_query::UserQuery, user_resolver::UserResolver};
pub mod task;
pub mod user;
#[derive(Default, MergedObject)]
pub struct Mutation(UserResolver, TaskResolver);

#[derive(Default, MergedObject)]
pub struct Query(UserQuery);
