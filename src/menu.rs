//! Menu page — coffee, pastries, light bites, lunch.
use crate::ui::{Heading, Link, Navigation, Region};
use fission::prelude::*;

#[derive(Clone)]
struct MenuItem {
    id: &'static str,
    name: &'static str,
    description: &'static str,
    price: &'static str,
}
impl From<MenuItem> for Widget {
    fn from(item: MenuItem) -> Self {
        Region(
            "menu-item",
            vec![
                Region(
                    "menu-item-copy",
                    vec![
                        Text::new(item.name)
                            .semantics_identifier(format!("menu-item-name:{}", item.id))
                            .into(),
                        Text::new(item.description).into(),
                    ],
                )
                .into(),
                Text::new(item.price)
                    .semantics_identifier(format!("menu-item-price:{}", item.id))
                    .into(),
            ],
        )
        .into()
    }
}

#[derive(Clone)]
struct MenuSection {
    id: &'static str,
    label: &'static str,
    heading: &'static str,
    note: Option<&'static str>,
    items: Vec<Widget>,
}
impl From<MenuSection> for Widget {
    fn from(s: MenuSection) -> Self {
        let mut children: Vec<Widget> = vec![
            Text::new(s.label).into(),
            Heading(2, s.id, s.heading).into(),
        ];
        if let Some(note) = s.note {
            children.push(Text::new(note).into());
        }
        children.extend(s.items);
        Region(s.id, children).into()
    }
}

#[derive(Clone)]
pub struct Menu;
impl From<Menu> for Widget {
    fn from(_: Menu) -> Self {
        Region(
            "page",
            vec![
                Navigation.into(),
                Region(
                    "page-hero",
                    vec![
                        Text::new("THE MENU").into(),
                        Heading(1, "menu-title", "Simple things,\ndone well.").into(),
                        Text::new(
                            "Everything is made in our kitchen. \
                             Seasonal changes are on the blackboard.",
                        )
                        .into(),
                    ],
                )
                .into(),
                // ── Coffee & drinks ───────────────────────────────────────
                MenuSection {
                    id: "section-coffee",
                    label: "01 / COFFEE & DRINKS",
                    heading: "Coffee",
                    note: Some(
                        "All espresso drinks use our house blend from Tíerra Roasters, \
                         medium-roasted in Thornfield. Milk alternatives +50p.",
                    ),
                    items: vec![
                        MenuItem {
                            id: "espresso",
                            name: "Espresso",
                            description: "Double shot, served with sparkling water.",
                            price: "£2.40",
                        }
                        .into(),
                        MenuItem {
                            id: "flat-white",
                            name: "Flat white",
                            description: "Two ristretto shots, steamed whole milk.",
                            price: "£3.20",
                        }
                        .into(),
                        MenuItem {
                            id: "oat-latte",
                            name: "Oat latte",
                            description: "Double shot, house-made oat milk.",
                            price: "£3.60",
                        }
                        .into(),
                        MenuItem {
                            id: "iced-honey-latte",
                            name: "Iced honey oat latte",
                            description: "Double shot, oat milk, wildflower honey, over ice.",
                            price: "£4.20",
                        }
                        .into(),
                        MenuItem {
                            id: "filter",
                            name: "Filter coffee",
                            description: "Single-origin rotating weekly. Ask at the counter.",
                            price: "£2.80",
                        }
                        .into(),
                        MenuItem {
                            id: "chai",
                            name: "Masala chai latte",
                            description: "House-spiced concentrate, steamed oat milk.",
                            price: "£3.60",
                        }
                        .into(),
                        MenuItem {
                            id: "lemonade",
                            name: "Sparkling lemonade",
                            description: "House-pressed lemon, elderflower, sparkling water.",
                            price: "£2.80",
                        }
                        .into(),
                    ],
                }
                .into(),
                // ── Pastries ──────────────────────────────────────────────
                MenuSection {
                    id: "section-bakery",
                    label: "02 / BAKERY",
                    heading: "Pastries & bakes",
                    note: Some(
                        "Baked from 5:30 am, available from opening until sold out. \
                         Vegan options marked (v).",
                    ),
                    items: vec![
                        MenuItem {
                            id: "cardamom-bun",
                            name: "Brown butter cardamom bun",
                            description: "Laminated dough, brown-butter frangipane, cardamom sugar.",
                            price: "£3.50",
                        }
                        .into(),
                        MenuItem {
                            id: "croissant",
                            name: "Plain croissant",
                            description: "Classic laminated butter pastry.",
                            price: "£2.80",
                        }
                        .into(),
                        MenuItem {
                            id: "almond-croissant",
                            name: "Almond croissant",
                            description: "Day-old croissant, almond cream, toasted flakes.",
                            price: "£3.20",
                        }
                        .into(),
                        MenuItem {
                            id: "banana-miso",
                            name: "Banana & miso loaf (v)",
                            description: "White miso caramel, dark chocolate chip.",
                            price: "£3.40",
                        }
                        .into(),
                        MenuItem {
                            id: "cheese-scone",
                            name: "Cheese & herb scone",
                            description: "Mature cheddar, chives, served warm with butter.",
                            price: "£2.90",
                        }
                        .into(),
                    ],
                }
                .into(),
                // ── Light bites ───────────────────────────────────────────
                MenuSection {
                    id: "section-bites",
                    label: "03 / LIGHT BITES",
                    heading: "Light bites",
                    note: Some("Served from opening until 3 pm."),
                    items: vec![
                        MenuItem {
                            id: "toast",
                            name: "Sourdough toast",
                            description: "Two slices, cultured butter, jam or Marmite.",
                            price: "£3.20",
                        }
                        .into(),
                        MenuItem {
                            id: "avocado",
                            name: "Smashed avocado on toast",
                            description: "Sourdough, avocado, chilli flakes, lemon, olive oil.",
                            price: "£7.50",
                        }
                        .into(),
                        MenuItem {
                            id: "poached-eggs",
                            name: "Poached eggs on toast",
                            description: "Two eggs, sourdough, hollandaise, chives.",
                            price: "£8.20",
                        }
                        .into(),
                    ],
                }
                .into(),
                // ── Lunch ─────────────────────────────────────────────────
                MenuSection {
                    id: "section-lunch",
                    label: "04 / LUNCH",
                    heading: "Lunch",
                    note: Some("Served noon – 3 pm, Monday to Saturday."),
                    items: vec![
                        MenuItem {
                            id: "soup",
                            name: "Soup of the week",
                            description: "Ask at the counter. Sourdough and cultured butter.",
                            price: "£6.50",
                        }
                        .into(),
                        MenuItem {
                            id: "sandwich",
                            name: "Doorstep sandwich",
                            description: "Thick-cut sourdough, seasonal filling, small side salad.",
                            price: "£8.50",
                        }
                        .into(),
                        MenuItem {
                            id: "grain-bowl",
                            name: "Grain bowl",
                            description: "Farro, roasted seasonal veg, tahini dressing, dukkah.",
                            price: "£9.00",
                        }
                        .into(),
                    ],
                }
                .into(),
                // ── Allergens ─────────────────────────────────────────────
                Region(
                    "allergens-notice",
                    vec![
                        Heading(2, "allergens", "Allergens & dietary needs").into(),
                        Text::new(
                            "Our kitchen handles gluten, dairy, eggs, nuts and sesame. \
                             Please speak to a member of staff before ordering if you \
                             have an allergy or intolerance.",
                        )
                        .into(),
                        Text::new(
                            "This is a demo site — do not rely on this information for \
                             real allergen decisions.",
                        )
                        .into(),
                    ],
                )
                .into(),
                // ── CTA ───────────────────────────────────────────────────
                Region(
                    "page-cta",
                    vec![
                        Heading(2, "menu-cta", "Ready to visit?").into(),
                        Text::new("Find our opening hours, address and directions.").into(),
                        Link("How to find us →", "/visit/").into(),
                    ],
                )
                .into(),
            ],
        )
        .into()
    }
}
