use godot::prelude::*;

mod game;
mod player;
mod platform;
mod pickup;

struct KnightPlatformer;

#[gdextension]
unsafe impl ExtensionLibrary for KnightPlatformer {}
