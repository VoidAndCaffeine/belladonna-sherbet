use serial_test::serial;
mod common;
use common::TestApp;
use belladonna_sherbet::plugins::yarn::{DialogueState, YarnNode};
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
#[serial]
fn test_dialogue_state_transition_on_yarn_collision() {
    let mut test_app = TestApp::new();
    
    
    test_app.spawn_player();
    test_app.advance_frames(5);

    // Initial state
    assert_eq!(test_app.get_dialogue_state(), DialogueState::Game);

    // Create YarnNode trigger
    let trigger_entity = test_app.app.world_mut().spawn(YarnNode::new("TestNode")).id();
    let player_entity = test_app.app.world_mut().query_filtered::<Entity, With<Player>>()
        .single(test_app.app.world_mut()).unwrap();

    let collision = make_collision(trigger_entity, player_entity);
    test_app.app.world_mut().trigger(collision);
    test_app.advance_frames(2);

    // Dialogue state should transition to Dialogue
    assert_eq!(test_app.get_dialogue_state(), DialogueState::Dialogue);
}

#[test]
#[serial]
fn test_dialogue_ignores_non_player_collision() {
    let mut test_app = TestApp::new();
    
    
    test_app.spawn_player();
    test_app.advance_frames(5);

    let trigger_entity = test_app.app.world_mut().spawn(YarnNode::new("TestNode")).id();
    let non_player_entity = test_app.app.world_mut().spawn_empty().id();

    let collision = make_collision(trigger_entity, non_player_entity);
    test_app.app.world_mut().trigger(collision);
    test_app.advance_frames(2);

    assert_eq!(test_app.get_dialogue_state(), DialogueState::Game);
}

#[test]
#[serial]
fn test_dialogue_ignores_missing_yarn_node() {
    let mut test_app = TestApp::new();
    
    
    test_app.spawn_player();
    test_app.advance_frames(5);

    let trigger_entity = test_app.app.world_mut().spawn_empty().id();
    let player_entity = test_app.app.world_mut().query_filtered::<Entity, With<Player>>()
        .single(test_app.app.world_mut()).unwrap();

    let collision = make_collision(trigger_entity, player_entity);
    test_app.app.world_mut().trigger(collision);
    test_app.advance_frames(2);

    assert_eq!(test_app.get_dialogue_state(), DialogueState::Game);
}