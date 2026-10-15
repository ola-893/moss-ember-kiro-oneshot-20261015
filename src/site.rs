//! Moss & Ember static website — entry point, routes, footer.

use crate::home::Home;
use crate::menu::Menu;
use crate::ui::Footer;
use crate::visit::Visit;
use fission::prelude::*;
use fission::site::FissionSite;

#[derive(Debug, Default)]
pub struct SiteState;
impl GlobalState for SiteState {}

pub fn site_app() -> FissionSite {
    FissionSite::new()
        .user_css(include_str!("site.css"))
        .route_widget::<SiteState, _>(
            "/",
            "Moss & Ember — Neighbourhood Café & Bakery",
            Some(
                "House-roasted coffee, seasonal pastries and honest lunches \
                 in Thornfield Village. Demo site."
                    .into(),
            ),
            Home,
        )
        .route_widget::<SiteState, _>(
            "/menu/",
            "Menu — Moss & Ember",
            Some("Coffee, pastries, light bites and lunch at Moss & Ember.".into()),
            Menu,
        )
        .route_widget::<SiteState, _>(
            "/visit/",
            "Visit — Moss & Ember",
            Some(
                "Opening hours, address and directions for Moss & Ember \
                 in Thornfield Village."
                    .into(),
            ),
            Visit,
        )
        .footer_widget::<SiteState, _>(Footer)
}
