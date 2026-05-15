use crate::components::team_grid::TeamGrid;
use crate::components::team_hero::TeamHero;
use crate::data::teams::ec_services_team;

use leptos::prelude::*;

#[component]
pub fn TeamECServices() -> impl IntoView {
    let team = ec_services_team();

    view! {
        <TeamHero
            team_name="Unified EC services team"
            description="Designing and managing implementation of a unified EC Services interface"
        />
        <TeamGrid members=team />
    }
}
