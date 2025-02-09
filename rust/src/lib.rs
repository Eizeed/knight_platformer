use godot::prelude::*;

mod game;
mod player;
mod platform;

struct KnightPlatformer;

#[gdextension]
unsafe impl ExtensionLibrary for KnightPlatformer {}
