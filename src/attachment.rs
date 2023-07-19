use crate::user::User;

#[derive(Debug)]
pub struct Attachment {
    id: String,
    name: String,
    create_time: u64,
    creator: User,
    hash: String,
    mime: String,
    size_byte: u64,
    descriptions: String,
}
