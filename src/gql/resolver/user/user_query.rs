use std::sync::Arc;

use crate::{
    gql::schema::{
        response_schema::SuccessMessage,
        user_schema::{UserCreateInput, UserResponse},
    },
    ApplicationState,
};
use async_graphql::{Context, Object, Result};

#[derive(Debug, Clone, Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_user_by_id(&self, ctx: &Context<'_>, body: i32) -> Result<SuccessMessage> {
        match ctx.data::<Arc<ApplicationState>>() {
            Ok(db) => {
                // let user_response: Option<bool> = sqlx::query_scalar(
                //     r#"SELECT 1 FROM tbl_user tu WHERE id = $1
                //         LEFT OUTER JOIN tbl_task tt ON tu.id = tt.user_id
                // )"#,
                // )
                // .bind(body.user_name.clone())
                // .fetch_one(&db.pg_pool)
                // .await
                // .map_err(|e| format!("{}", e))?;
                // if let Some(exists) = is_present {
                //     if exists {
                //         return Err("User already exists".into());
                //     }
                // }
                // sqlx::query_as(
                //     r#"INSERT INTO
                // tbl_user (
                //      name,
                //      user_name
                // )
                // VALUES
                // ($1,$2)"#,
                // )
                // .bind(body.name)
                // .bind(body.user_name)
                // .fetch_one(&db.pg_pool)
                // .await
                // .map_err(|e| format!("{}", e))?;
                Ok(SuccessMessage {
                    message: "Created".into(),
                })
            }
            Err(err) => return Err(err),
        }
    }
}
