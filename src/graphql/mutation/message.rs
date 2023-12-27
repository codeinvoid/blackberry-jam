use async_graphql::{Context, InputObject, Object, Result};

use crate::{
    prisma::{user, PrismaClient},
    graphql::types::Message,
};

// I normally separate the input types into separate files/modules, but this is just
// a quick example.

#[derive(InputObject)]
pub struct CreateMessageInput {
    content: String,
    user_id: String,
}

#[derive(Default)]
pub struct MessageMutation;

#[Object]
impl MessageMutation {
    pub async fn create_message(&self, ctx: &Context<'_>, input: CreateMessageInput) -> Result<Message> {
        let db = ctx.data::<PrismaClient>().unwrap();

        let created = db
            .message()
            .create(input.content, user::UniqueWhereParam::IdEquals(input.user_id), vec![])
            .exec()
            .await?;

        Ok(created.into())
    }
}