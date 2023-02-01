mod does_not_exist;
mod home;
mod home_content;
mod not_found;
mod contact;
mod submitted;

pub use self::{does_not_exist::DoesNotExist, home::Home, not_found::NotFound, contact::Contact, submitted::Submitted};
