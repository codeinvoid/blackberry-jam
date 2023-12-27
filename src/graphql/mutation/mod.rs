pub mod user;
pub mod banned;
pub mod message;
pub mod report;

pub use banned::BannedMutation;
pub use user::UserMutation;
pub use message::MessageMutation;
pub use report::ReportMutation;

// Add your other ones here to create a unified Mutation object
// e.x. Mutation(PostMutation, OtherMutation, OtherOtherMutation)
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(BannedMutation, UserMutation, MessageMutation, ReportMutation);
