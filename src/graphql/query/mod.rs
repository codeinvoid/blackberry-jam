pub mod banned;
pub mod user;
pub mod message;
pub mod report;

pub use banned::BannedQuery;
pub use user::UserQuery;
pub use message::MessageQuery;
pub use report::ReportQuery;

// Add your other ones here to create a unified Query object
// e.x. Query(PostQuery, OtherQuery, OtherOtherQuery)
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(BannedQuery, UserQuery, ReportQuery, MessageQuery);
