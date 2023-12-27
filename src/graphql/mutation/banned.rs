use async_graphql::{Context, InputObject, Object, Result};

use crate::{
    graphql::types::Banned,
    prisma::{user, PrismaClient},
};

// I normally separate the input types into separate files/modules, but this is just
// a quick example.

#[derive(InputObject)]
pub struct CreateBannedInput {
    pub is_active: bool,
    pub user_id: String,
    pub reason: String,
    pub finish: i64,
    pub processor: String,
}

#[derive(Default)]
pub struct BannedMutation;

#[Object]
impl BannedMutation {
    pub async fn create_banned(
        &self,
        ctx: &Context<'_>,
        input: CreateBannedInput,
    ) -> Result<Banned> {
        let db = ctx.data::<PrismaClient>().unwrap();

        let created = db
            .banned()
            .create(
                input.is_active,
                input.reason,
                input.finish,
                input.processor,
                user::id::equals(input.user_id),
                vec![],
            )
            .exec()
            .await?;

        Ok(created.into())
    }
}
