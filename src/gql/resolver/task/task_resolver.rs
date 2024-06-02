use std::sync::Arc;

use async_graphql::{Context, Object, Result};

use crate::{
    gql::schema::{response_schema::SuccessMessage, task_schema::TaskInput},
    ApplicationState,
};

#[derive(Debug, Clone, Default)]
pub struct TaskResolver;

#[Object]
impl TaskResolver {
    async fn create_task(&self, ctx: &Context<'_>, body: TaskInput) -> Result<SuccessMessage> {
        match ctx.data::<Arc<ApplicationState>>() {
            Ok(db) => {
                sqlx::query_as(
                    r#"INSERT INTO
                tbl_task (
                    user_id,
                    title,
                    description
                )
                VALUES
                ($1,$2,$3) returning *"#,
                )
                .bind(body.user_id)
                .bind(body.title)
                .bind(body.description)
                .fetch_one(&db.pg_pool)
                .await
                .map_err(|e| format!("{}", e))?;
                Ok(SuccessMessage {
                    message: "Created".into(),
                })
            }
            Err(err) => return Err(err),
        }
    }
}
