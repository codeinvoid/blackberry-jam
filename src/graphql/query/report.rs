use async_graphql::{Context, Object, Result};

use crate::{graphql::types::Report, prisma::PrismaClient};

#[derive(Default)]
pub struct ReportQuery;

#[Object]
impl ReportQuery {
    async fn get_reports(&self, ctx: &Context<'_>) -> Result<Vec<Report>> {
        let db = ctx.data::<PrismaClient>().unwrap();

        Ok(db
            .report()
            .find_many(vec![])
            .exec()
            .await?
            .into_iter()
            .map(|p| p.into())
            .collect())
    }
}
