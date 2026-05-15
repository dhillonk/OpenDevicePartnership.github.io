use crate::components::team_grid::TeamGrid;
use crate::components::team_hero::TeamHero;
use crate::data::teams::patina_team;

use leptos::prelude::*;

#[component]
pub fn TeamPatina() -> impl IntoView {
    let team = patina_team();

    view! {
        <TeamHero
            team_name="Patina team"
            description="Developing and managing development of a new modern UEFI"
        />
        <TeamGrid members=team />
    }
}
