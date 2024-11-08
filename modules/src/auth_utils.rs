// bring models.rs submod into scope
pub mod models;

// module is stored in file but submodules are stored in the folder, with same name as the folder
fn logout() {
    // logout
}
pub fn login(credentials: models::Credentials) { // just public in the crate not to the outside world
    // authetication
    crate::database::get_user()
}
