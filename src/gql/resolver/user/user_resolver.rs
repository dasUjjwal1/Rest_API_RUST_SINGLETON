use std::sync::Arc;

use async_graphql::{Context, Object, Result};

use crate::{
    gql::schema::{response_schema::SuccessMessage, user_schema::UserCreateInput},
    ApplicationState,
};

#[derive(Debug, Clone, Default)]
pub struct UserResolver;

#[Object]
impl UserResolver {
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        body: UserCreateInput,
    ) -> Result<SuccessMessage> {
        match ctx.data::<Arc<ApplicationState>>() {
            Ok(db) => {
                let is_present: Option<bool> = sqlx::query_scalar(
                    r#"SELECT EXISTS(SELECT 1 FROM tbl_user WHERE user_name = $1)"#,
                )
                .bind(body.user_name.clone())
                .fetch_one(&db.pg_pool)
                .await
                .map_err(|e| format!("{}", e))?;
                if let Some(exists) = is_present {
                    if exists {
                        return Err("User already exists".into());
                    }
                }
                sqlx::query_as(
                    r#"INSERT INTO
                tbl_user (
                     name,
                     user_name
                )
                VALUES
                ($1,$2) RETURNING *"#,
                )
                .bind(body.name)
                .bind(body.user_name)
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
