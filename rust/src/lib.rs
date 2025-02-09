use godot::prelude::*;

mod game;
mod player;

struct KnightPlatformer;

#[gdextension]
unsafe impl ExtensionLibrary for KnightPlatformer {}
