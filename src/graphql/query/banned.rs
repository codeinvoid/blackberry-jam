use async_graphql::{Context, Object, Result};

use crate::{graphql::types::Banned, prisma::PrismaClient};

#[derive(Default)]
pub struct BannedQuery;

#[Object]
impl BannedQuery {
    async fn get_banneds(&self, ctx: &Context<'_>) -> Result<Vec<Banned>> {
        let db = ctx.data::<PrismaClient>().unwrap();

        Ok(db
            .banned()
            .find_many(vec![])
            .exec()
            .await?
            .into_iter()
            .map(|p| p.into())
            .collect())
    }
}
