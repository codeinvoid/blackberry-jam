pub mod banned;
pub mod message;
pub mod report;
pub mod user;

pub use banned::BannedMutation;
pub use message::MessageMutation;
pub use report::ReportMutation;
pub use user::UserMutation;

// Add your other ones here to create a unified Mutation object
// e.x. Mutation(PostMutation, OtherMutation, OtherOtherMutation)
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(
    BannedMutation,
    UserMutation,
    MessageMutation,
    ReportMutation,
);
