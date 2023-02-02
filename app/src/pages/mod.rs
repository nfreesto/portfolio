mod contact;
mod does_not_exist;
mod home;
mod home_content;
mod not_found;
mod submitted;

pub use self::{
    contact::Contact, does_not_exist::DoesNotExist, home::Home, not_found::NotFound,
    submitted::Submitted,
};
