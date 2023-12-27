use async_graphql::{Context, InputObject, Object, Result};

use crate::{graphql::types::User, prisma::PrismaClient};

#[derive(InputObject)]
pub struct CreateUserInput {
    pub id: String,
    pub q_id: i64,
    pub is_active: bool,
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    pub async fn create_user(&self, ctx: &Context<'_>, input: CreateUserInput) -> Result<User> {
        let db = ctx.data::<PrismaClient>().unwrap();

        let created = db
            .user()
            .create(input.id, input.q_id, input.is_active, vec![])
            .exec()
            .await?;

        Ok(created.into())
    }
}
