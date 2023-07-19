use crate::status::Status;
use crate::user::User;

#[derive(Debug)]
pub struct Project {
    id: String,
    name: String,
    create_time: u64,
    owner: User,
    creator: User,
    status: Status,
    status_category: String,
    stick_to_top: bool,
    is_archive: bool,
    archive_user: User,
    archive_time: u64,
    planned_start_date: u64,
    planned_end_date: u64,
}
