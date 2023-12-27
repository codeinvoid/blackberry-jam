use async_graphql::{Context, InputObject, Object, Result};

use crate::{
    graphql::types::Report,
    prisma::{user, PrismaClient, ProgressType, ReportType},
};

// I normally separate the input types into separate files/modules, but this is just
// a quick example.

#[derive(InputObject)]
pub struct CreateReportInput {
    r#type: ReportType,
    progress: ProgressType,
    user_id: String,
    title: String,
    reason: String,
}

#[derive(Default)]
pub struct ReportMutation;

#[Object]
impl ReportMutation {
    pub async fn create_report(
        &self,
        ctx: &Context<'_>,
        input: CreateReportInput,
    ) -> Result<Report> {
        let db = ctx.data::<PrismaClient>().unwrap();

        let created = db
            .report()
            .create(
                input.r#type,
                input.title,
                input.progress,
                input.reason,
                user::UniqueWhereParam::IdEquals(input.user_id),
                vec![],
            )
            .exec()
            .await?;

        Ok(created.into())
    }
}
