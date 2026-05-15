use crate::components::team_grid::TeamGrid;
use crate::components::team_hero::TeamHero;
use crate::data::teams::ec_team;

use leptos::prelude::*;

#[component]
pub fn TeamEC() -> impl IntoView {
    let team = ec_team();

    view! {
        <TeamHero
            team_name="Secure EC team"
            description="Developing and managing secure EC internals"
        />
        <TeamGrid members=team />
    }
}
