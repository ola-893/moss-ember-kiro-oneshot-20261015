//! Shared design widgets for Moss & Ember.
//!
//! Palette  cream #f7f4ef · forest-green #2d5a3d · terracotta #c8603a
//!          sand #e8ddd0 · muted #7a6b5d · rule #d9d0c5
use fission::prelude::*;

// ── Low-level helpers ─────────────────────────────────────────────────────

/// A semantics-tagged Column.
#[derive(Clone)]
pub struct Region(pub &'static str, pub Vec<Widget>);
impl From<Region> for Widget {
    fn from(r: Region) -> Self {
        SemanticsRegion::new(Column {
            children: r.1,
            gap: Some(20.0),
            ..Default::default()
        })
        .identifier(r.0)
        .into()
    }
}

/// Semantic heading text (level 1–3, css-id, display text).
#[derive(Clone)]
pub struct Heading(pub u8, pub &'static str, pub &'static str);
impl From<Heading> for Widget {
    fn from(h: Heading) -> Self {
        Text::new(h.2)
            .semantics_identifier(format!("site-heading-{}:{}", h.0, h.1))
            .into()
    }
}

/// Internal or external hyperlink.
#[derive(Clone)]
pub struct Link(pub &'static str, pub &'static str);
impl From<Link> for Widget {
    fn from(l: Link) -> Self {
        Text::new(l.0)
            .semantics_identifier(format!("markdown-link:{}", l.1))
            .into()
    }
}

// ── Navigation ────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct Navigation;
impl From<Navigation> for Widget {
    fn from(_: Navigation) -> Self {
        Region(
            "navigation",
            vec![
                Link("moss & ember", "/").into(),
                Region(
                    "nav-links",
                    vec![
                        Link("Menu", "/menu/").into(),
                        Link("Visit & hours", "/visit/").into(),
                    ],
                )
                .into(),
            ],
        )
        .into()
    }
}

// ── Footer ────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct Footer;
impl From<Footer> for Widget {
    fn from(_: Footer) -> Self {
        Region(
            "footer",
            vec![
                Region(
                    "footer-brand",
                    vec![
                        Text::new("Moss & Ember").into(),
                        Text::new(
                            "14 Ferndale Road, Thornfield Village · \
                             Open daily 7 am – 5 pm · Demo site",
                        )
                        .into(),
                    ],
                )
                .into(),
                Region(
                    "footer-links",
                    vec![
                        Link("Menu", "/menu/").into(),
                        Link("Visit & hours", "/visit/").into(),
                        Link("Built with Fission", "https://fission.rs").into(),
                    ],
                )
                .into(),
            ],
        )
        .into()
    }
}
