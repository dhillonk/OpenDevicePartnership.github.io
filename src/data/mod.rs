//! Static content separated from UI components.
//!
//! These modules hold rosters, partner lists, and project copy that
//! were previously inlined into page components. Keeping the data
//! here lets contributors edit who appears on which team without
//! touching layout code.

pub mod announcements;
pub mod partners;
pub mod projects;
pub mod teams;
