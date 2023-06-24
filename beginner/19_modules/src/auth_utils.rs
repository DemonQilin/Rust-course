pub mod models;

// Path relativo
pub fn login(creds: models::Credentials) {
    // authenticate...
    // Path absoluto
    crate::database::get_user();
}

fn logout() {
    // log user out...
}
