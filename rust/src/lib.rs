use godot::prelude::*;

mod game;
mod player;
mod platform;
mod pickup;
mod killzone;
mod enemy;
mod gamemanager;

struct KnightPlatformer;

#[gdextension]
unsafe impl ExtensionLibrary for KnightPlatformer {}
