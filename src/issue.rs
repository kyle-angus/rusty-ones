use crate::attachment::Attachment;
use crate::project::Project;
use crate::status::Status;
use crate::user::User;

#[derive(Debug)]
pub struct IssueParent {
    id: String,
    title: String,
}

#[derive(Debug)]
pub struct IssueType {
    id: String,
    name: String,
    is_subissue_type: bool,
    issue_type: String,
    creat_time: u64,
    build_in: bool,
    project: Project,
}

#[derive(Debug)]
pub struct IssuePriority {
    id: String,
    name: String,
}

#[derive(Debug)]
pub struct IssueWorkflow {
    id: String,
    name: String,
    start: String,
    end: String,
}

#[derive(Debug)]
pub struct Issue {
    assignee: String,
    create_time: u64,
    due_date: u64,
    time_estimated_hours: u64,
    issue_type: IssueType,
    title: String,
    number: u64,
    creator: User,
    parent: IssueParent,
    priority: IssuePriority,
    project: Project,
    time_remaining_hours: u64,
    sprint: Sprint,
    status: Status,
    subissue_type: SubIssueType,
    subissue_count: u64,
    subissue_done_count: u64,
    subissues: Vec<SubIssue>,
    total_time_estimated_hour: u64,
    time_spent_hour: u64,
    total_time_remaining_hour: u64,
    id: String,
    attachments: Vec<Attachment>,
}

#[derive(Debug)]
pub struct Sprint {
    id: String,
    name: String,
}

#[derive(Debug)]
pub struct SubIssue {
    id: String,
    name: String,
}

#[derive(Debug)]
pub struct SubIssueType {
    id: String,
    name: String,
}
