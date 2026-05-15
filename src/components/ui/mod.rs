//! Design system primitives.
//!
//! `ui::*` collects the small, opinionated building blocks that the
//! pages and feature components are encouraged to compose instead of
//! reaching for raw Tailwind utility classes. Each primitive:
//!
//!  * resolves to a single semantic HTML element where possible,
//!  * exposes a `class` escape hatch for one-off overrides,
//!  * factors its class composition through a pure helper that is
//!    pinned by host-side unit tests.
//!
//! Existing higher-level shells ([`crate::components::section::Section`]
//! and [`crate::components::themed_icon::ThemedIcon`]) are re-exported
//! here so a single `use crate::components::ui::*;` is enough at most
//! call sites.
//!
//! NOTE: this module is intentionally introduced ahead of any call
//! sites so the migration can land in follow-up commits without
//! mixing the design-system rollout into a single oversized diff.
//! The blanket allow below silences the dead-code and unused-import
//! lints for variants and re-exports that are not yet wired up.
//! Remove it once every primitive is in active use.
#![allow(dead_code, unused_imports)]

mod arrow_link;
mod button;
mod container;
mod doc_link_item;
mod grid;
mod heading;
mod icon_block;
mod labeled_section;
mod link;
mod mono;
mod stack;
mod text;
mod two_column_intro;
mod value_prop_card;

pub use arrow_link::{ArrowLink, ArrowLinkSize};
pub use button::{Button, ButtonVariant, IconButton};
pub use container::Container;
pub use doc_link_item::DocLinkItem;
pub use grid::Grid;
pub use heading::{Heading, HeadingLevel};
pub use icon_block::{IconBlock, IconBlockSize};
pub use labeled_section::LabeledSection;
pub use link::{Link, LinkSize};
pub use mono::{Mono, MonoSize};
pub use stack::{Stack, StackDirection};
pub use text::{Text, TextSize};
pub use two_column_intro::TwoColumnIntro;
pub use value_prop_card::ValuePropCard;

// Re-exports so callers can pull every primitive from a single path.
pub use crate::components::section::{Section, Surface};
pub use crate::components::themed_icon::ThemedIcon;
