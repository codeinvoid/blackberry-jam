use crate::prisma::{banned, message, report, user, PrismaClient, ProgressType, ReportType};
use async_graphql::{ComplexObject, Context, Result, SimpleObject};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct User {
    pub id: String,
    pub q_id: i64,
    pub is_active: bool,
}

#[ComplexObject]
impl User {
    pub async fn banned(&self, ctx: &Context<'_>) -> Result<Vec<Banned>> {
        let db = ctx.data::<PrismaClient>().unwrap();

        Ok(db
            .banned()
            .find_many(vec![banned::u_id::equals(self.id.clone())])
            .exec()
            .await?
            .into_iter()
            .map(|p| p.into())
            .collect())
    }

    pub async fn message(&self, ctx: &Context<'_>) -> Result<Vec<Message>> {
        let db = ctx.data::<PrismaClient>().unwrap();

        Ok(db
            .message()
            .find_many(vec![message::u_id::equals(self.id.clone())])
            .exec()
            .await?
            .into_iter()
            .map(|p| p.into())
            .collect())
    }

    pub async fn report(&self, ctx: &Context<'_>) -> Result<Vec<Report>> {
        let db = ctx.data::<PrismaClient>().unwrap();

        Ok(db
            .report()
            .find_many(vec![report::u_id::equals(self.id.clone())])
            .exec()
            .await?
            .into_iter()
            .map(|p| p.into())
            .collect())
    }
}

impl Into<User> for user::Data {
    fn into(self) -> User {
        User {
            id: self.id,
            q_id: self.q_id,
            is_active: self.is_active,
        }
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Banned {
    pub id: String,
    pub is_active: bool,
    pub user_id: String,
    pub reason: String,
    pub finish: i64,
    pub processor: String,
}

#[ComplexObject]
impl Banned {
    pub async fn user(&self, ctx: &Context<'_>) -> Result<Option<Box<User>>> {
        let db = ctx.data::<PrismaClient>().unwrap();

        Ok(db
            .user()
            .find_unique(user::id::equals(self.user_id.clone()))
            .exec()
            .await?
            .map(|u| Box::new(u.into())))
    }
}

impl Into<Banned> for banned::Data {
    fn into(self) -> Banned {
        Banned {
            id: self.id,
            is_active: self.is_active,
            user_id: self.u_id,
            reason: self.reason,
            finish: self.finish,
            processor: self.processor,
        }
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Message {
    id: String,
    content: String,
    user_id: String,
}

#[ComplexObject]
impl Message {
    pub async fn user(&self, ctx: &Context<'_>) -> Result<Option<Box<User>>> {
        let db = ctx.data::<PrismaClient>().unwrap();

        Ok(db
            .user()
            .find_unique(user::id::equals(self.user_id.clone()))
            .exec()
            .await?
            .map(|u| Box::new(u.into())))
    }
}

impl Into<Message> for message::Data {
    fn into(self) -> Message {
        Message {
            id: self.id,
            content: self.content,
            user_id: self.u_id,
        }
    }
}

// #[derive(Enum, Copy, Clone, Eq, PartialEq)]
// pub enum ReportType {
//     REPORT,
//     ADVICE,
//     BUG
// }

// #[derive(Enum, Copy, Clone, Eq, PartialEq)]
// pub enum ProgressType {
//     SUBMIT,
//     PROCESS,
//     REJECT,
//     ACCEPT,
//     FINISH
// }

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Report {
    pub id: String,
    pub r#type: ReportType,
    pub title: String,
    pub progress: ProgressType,
    pub reason: String,
    pub report_name: Option<String>,
    pub user_id: String,
}

#[ComplexObject]
impl Report {
    pub async fn user(&self, ctx: &Context<'_>) -> Result<Option<Box<User>>> {
        let db = ctx.data::<PrismaClient>().unwrap();

        Ok(db
            .user()
            .find_unique(user::id::equals(self.user_id.clone()))
            .exec()
            .await?
            .map(|u| Box::new(u.into())))
    }
}

impl Into<Report> for report::Data {
    fn into(self) -> Report {
        Report {
            id: self.id,
            r#type: self.r#type,
            title: self.title,
            progress: self.progress,
            reason: self.reason,
            report_name: self.report_name,
            user_id: self.u_id,
        }
    }
}
