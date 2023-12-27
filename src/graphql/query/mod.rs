pub mod banned;
pub mod message;
pub mod report;
pub mod user;

pub use banned::BannedQuery;
pub use message::MessageQuery;
pub use report::ReportQuery;
pub use user::UserQuery;

// Add your other ones here to create a unified Query object
// e.x. Query(PostQuery, OtherQuery, OtherOtherQuery)
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(BannedQuery, UserQuery, ReportQuery, MessageQuery);
