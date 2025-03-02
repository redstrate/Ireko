pub mod generic;

mod localprofile;
pub use self::localprofile::LocalProfileObject;

mod persistent;
pub use self::persistent::PersistentObject;

mod slot;
pub use self::slot::SlotObject;
