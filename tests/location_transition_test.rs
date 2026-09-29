mod common;
use common::TestApp;
use belladonna_sherbet::plugins::location_change::{Location, LocationChange, LocationChangeInfo, LocationChangeDest};
use belladonna_sherbet::plugins::player::Player;
use bevy::prelude::*;
use avian3d::prelude::CollisionStart;

fn make_collision(collider1: Entity, collider2: Entity) -> CollisionStart {
    CollisionStart {
        collider1,
        collider2,
        body1: None,
        body2: None,
    }
}

#[test]
fn test_location_transition_on_player_collision() {
    let mut test_app = TestApp::new();
    test_app.spawn_player();
    test_app.advance_frames(5);

    let trigger_entity = test_app.app.world_mut().spawn(LocationChangeInfo {
        origin: Location::Test,
        destination: Location::Greenhouse,
    }).id();

    let player_entity = test_app.app.world().query_filtered::<Entity, With<Player>>()
        .single(test_app.app.world()).unwrap();

    let collision = make_collision(trigger_entity, player_entity);
    test_app.app.world_mut().trigger(collision);
    test_app.advance_frames(2);

    let location_change = test_app.app.world().resource::<LocationChange>();
    assert_eq!(location_change.destination, Location::Greenhouse);
}

#[test]
fn test_location_transition_ignores_non_player() {
    let mut test_app = TestApp::new();
    test_app.spawn_player();
    test_app.advance_frames(5);

    let trigger_entity = test_app.app.world_mut().spawn(LocationChangeInfo {
        origin: Location::Test,
        destination: Location::Greenhouse,
    }).id();

    let non_player_entity = test_app.app.world_mut().spawn_empty().id();

    let collision = make_collision(trigger_entity, non_player_entity);
    test_app.app.world_mut().trigger(collision);
    test_app.advance_frames(2);

    let location_change = test_app.app.world().resource::<LocationChange>();
    assert_eq!(location_change.destination, Location::Test);
}

#[test]
fn test_location_transition_updates_origin() {
    let mut test_app = TestApp::new();
    test_app.spawn_player();
    test_app.advance_frames(5);

    let trigger1 = test_app.app.world_mut().spawn(LocationChangeInfo {
        origin: Location::Test,
        destination: Location::Greenhouse,
    }).id();

    let player_entity = test_app.app.world().query_filtered::<Entity, With<Player>>()
        .single(test_app.app.world()).unwrap();

    let collision1 = make_collision(trigger1, player_entity);
    test_app.app.world_mut().trigger(collision1);
    test_app.advance_frames(2);

    let location_change = test_app.app.world().resource::<LocationChange>();
    assert_eq!(location_change.origin, Location::Test);
    assert_eq!(location_change.destination, Location::Greenhouse);

    // Second transition
    let trigger2 = test_app.app.world_mut().spawn(LocationChangeInfo {
        origin: Location::Greenhouse,
        destination: Location::CaspianGround,
    }).id();

    let collision2 = make_collision(trigger2, player_entity);
    test_app.app.world_mut().trigger(collision2);
    test_app.advance_frames(2);

    let location_change = test_app.app.world().resource::<LocationChange>();
    assert_eq!(location_change.origin, Location::Greenhouse);
    assert_eq!(location_change.destination, Location::CaspianGround);
}

#[test]
fn test_location_transition_with_missing_info_ignored() {
    let mut test_app = TestApp::new();
    test_app.spawn_player();
    test_app.advance_frames(5);

    let trigger_entity = test_app.app.world_mut().spawn_empty().id();
    let player_entity = test_app.app.world().query_filtered::<Entity, With<Player>>()
        .single(test_app.app.world()).unwrap();

    let collision = make_collision(trigger_entity, player_entity);
    test_app.app.world_mut().trigger(collision);
    test_app.advance_frames(2);

    let location_change = test_app.app.world().resource::<LocationChange>();
    assert_eq!(location_change.destination, Location::Test);
}