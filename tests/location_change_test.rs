use belladonna_sherbet::plugins::location_change::{Location, LocationChange, LocationChangeInfo, LocationChangeDest, change_location};
use belladonna_sherbet::plugins::player::Player;
use bevy::prelude::*;
use avian3d::prelude::CollisionStart;

fn setup_test_app() -> App {
    let mut app = App::new();
    app.init_resource::<LocationChange>()
        .add_observer(change_location);
    app
}

#[test]
fn test_location_enum_default() {
    let location = Location::default();
    assert_eq!(location, Location::Test);
}

#[test]
fn test_location_enum_variants() {
    assert_eq!(Location::Test as u8, 0);
    assert_eq!(Location::Greenhouse as u8, 1);
    assert_eq!(Location::CaspianGround as u8, 2);
}

#[test]
fn test_location_change_resource_default() {
    let mut app = setup_test_app();
    let location_change = app.world().resource::<LocationChange>();
    assert_eq!(location_change.origin, Location::Test);
    assert_eq!(location_change.destination, Location::Test);
}

#[test]
fn test_location_change_info_component() {
    let info = LocationChangeInfo {
        origin: Location::Test,
        destination: Location::Greenhouse,
    };
    assert_eq!(info.origin, Location::Test);
    assert_eq!(info.destination, Location::Greenhouse);
}

#[test]
fn test_location_change_dest_component() {
    let dest = LocationChangeDest {
        origin: Location::Test,
        destination: Location::Greenhouse,
    };
    assert_eq!(dest.origin, Location::Test);
    assert_eq!(dest.destination, Location::Greenhouse);
}

fn make_collision(collider1: Entity, collider2: Entity) -> CollisionStart {
    CollisionStart {
        collider1,
        collider2,
        body1: None,
        body2: None,
    }
}

#[test]
fn test_change_location_updates_resource() {
    let mut app = setup_test_app();

    let trigger_entity = app.world_mut().spawn(LocationChangeInfo {
        origin: Location::Test,
        destination: Location::Greenhouse,
    }).id();

    let player_entity = app.world_mut().spawn(Player).id();

    let collision_event = make_collision(trigger_entity, player_entity);
    app.world_mut().trigger(collision_event);
    app.update();

    let location_change = app.world().resource::<LocationChange>();
    assert_eq!(location_change.origin, Location::Test);
    assert_eq!(location_change.destination, Location::Greenhouse);
}

#[test]
fn test_change_location_ignores_non_player() {
    let mut app = setup_test_app();

    let trigger_entity = app.world_mut().spawn(LocationChangeInfo {
        origin: Location::Test,
        destination: Location::Greenhouse,
    }).id();

    let non_player_entity = app.world_mut().spawn_empty().id();

    let collision_event = make_collision(trigger_entity, non_player_entity);
    app.world_mut().trigger(collision_event);
    app.update();

    let location_change = app.world().resource::<LocationChange>();
    assert_eq!(location_change.origin, Location::Test);
    assert_eq!(location_change.destination, Location::Test);
}

#[test]
fn test_change_location_ignores_missing_info() {
    let mut app = setup_test_app();

    let trigger_entity = app.world_mut().spawn_empty().id();
    let player_entity = app.world_mut().spawn(Player).id();

    let collision_event = make_collision(trigger_entity, player_entity);
    app.world_mut().trigger(collision_event);
    app.update();

    let location_change = app.world().resource::<LocationChange>();
    assert_eq!(location_change.origin, Location::Test);
    assert_eq!(location_change.destination, Location::Test);
}

#[test]
fn test_location_change_multiple_transitions() {
    let mut app = setup_test_app();

    let trigger1 = app.world_mut().spawn(LocationChangeInfo {
        origin: Location::Test,
        destination: Location::Greenhouse,
    }).id();
    let player = app.world_mut().spawn(Player).id();

    let collision1 = make_collision(trigger1, player);
    app.world_mut().trigger(collision1);
    app.update();

    let location_change = app.world().resource::<LocationChange>();
    assert_eq!(location_change.destination, Location::Greenhouse);

    let trigger2 = app.world_mut().spawn(LocationChangeInfo {
        origin: Location::Greenhouse,
        destination: Location::CaspianGround,
    }).id();

    let collision2 = make_collision(trigger2, player);
    app.world_mut().trigger(collision2);
    app.update();

    let location_change = app.world().resource::<LocationChange>();
    assert_eq!(location_change.origin, Location::Greenhouse);
    assert_eq!(location_change.destination, Location::CaspianGround);
}