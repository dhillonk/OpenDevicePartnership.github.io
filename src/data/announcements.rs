//! Announcement metadata.
//!
//! Each [`Announcement`] is one row in the announcements page sidebar
//! and one selectable detail panel. The detail content (the
//! press-release prose) stays co-located with the page component
//! that renders it -- this module only owns the *metadata* so adding
//! a new announcement is a one-line struct literal here plus one
//! match-arm in the page.

/// One announcement entry.
#[derive(Clone, Copy)]
pub struct Announcement {
    /// URL slug used in `/announcements?id={slug}`. Must be unique
    /// and stable -- announcement permalinks rely on it.
    pub slug: &'static str,
    /// Short date-prefixed label shown in the sidebar list.
    pub link_label: &'static str,
    /// Full title shown above the detail panel content.
    pub title: &'static str,
}

/// Canonical announcement list, in the order shown in the sidebar.
pub const ANNOUNCEMENTS: &[Announcement] = &[Announcement {
    slug: "welcome-patina-announcement",
    link_label: "Oct-7-2025 Welcome Patina!",
    title: "Patina Project to Launch at UEFI 2025 Developer Conference & Plugfest",
}];

/// Look up an announcement index by slug. Returns `None` if no
/// announcement with that slug exists.
pub fn index_of(slug: &str) -> Option<usize> {
    ANNOUNCEMENTS.iter().position(|a| a.slug == slug)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn welcome_patina_is_first() {
        assert_eq!(ANNOUNCEMENTS[0].slug, "welcome-patina-announcement");
        assert_eq!(index_of("welcome-patina-announcement"), Some(0));
    }

    #[test]
    fn unknown_slug_returns_none() {
        assert_eq!(index_of("does-not-exist"), None);
    }

    #[test]
    fn slugs_are_unique() {
        let mut slugs: Vec<&str> = ANNOUNCEMENTS.iter().map(|a| a.slug).collect();
        slugs.sort();
        let len_before = slugs.len();
        slugs.dedup();
        assert_eq!(slugs.len(), len_before, "announcement slugs must be unique");
    }
}
