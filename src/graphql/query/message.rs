use async_graphql::{Context, Object, Result};

use crate::{prisma::PrismaClient, graphql::types::Message};

#[derive(Default)]
pub struct MessageQuery;

#[Object]
impl MessageQuery {
    async fn get_messages(&self, ctx: &Context<'_>) -> Result<Vec<Message>> {
        let db = ctx.data::<PrismaClient>().unwrap();

        Ok(db
            .message()
            .find_many(vec![])
            .exec()
            .await?
            .into_iter()
            .map(|p| p.into())
            .collect())
    }
}