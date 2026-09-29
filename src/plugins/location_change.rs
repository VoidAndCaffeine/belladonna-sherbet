use avian3d::prelude::CollisionStart;
use bevy::prelude::*;
use crate::plugins::player::Player;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[derive(Default,States,Clone,Copy,Eq,PartialEq,Debug,Hash)]
pub enum Location {
    #[default]
    Test,
    Greenhouse,
    CaspianGround,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct LocationChangeInfo{
    pub origin: Location,
    pub destination: Location,
}
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct LocationChangeDest{
    pub origin: Location,
    pub destination: Location,
}

#[derive(Resource,Reflect, Default)]
#[reflect(Resource)]
pub struct LocationChange{
    pub origin: Location,
    pub destination: Location,
}

pub struct LocationChangePlugin;

impl Plugin for LocationChangePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<Location>()
            .insert_resource(LocationChange { origin: Location::Test, destination: Location::Test })
            .register_type::<Location>()
            .register_type::<LocationChangeInfo>()
            .register_type::<LocationChangeDest>()
            .add_observer(change_location)
        ;
    }
}

pub fn change_location(
    event: On<CollisionStart>,
    query: Query<&LocationChangeInfo>,
    player_query: Query<&Player>,
    mut lc_change: ResMut<LocationChange>,
){
    let lc_info = match query.get(event.event_target()) {
        Ok(loc_info) => loc_info,
        Err(_) => return,
    };
    let _ = match player_query.get(event.collider2) {
        Ok(_) => {info!("Player entered doorway")}
        Err(_) => return,
    };
    info!("Change location requested from {:?} to {:?}",lc_info.origin,lc_info.destination);
    lc_change.destination = lc_info.destination;
    lc_change.origin = lc_info.origin;
}