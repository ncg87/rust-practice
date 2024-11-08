pub enum Status {
    Success,
    Failure,
}
pub fn connect_to_database(credentials: &crate::auth_utils::models::Credentials) -> Status {
    return Status::Success;
}
pub fn get_user() {
    //get user from database
}
