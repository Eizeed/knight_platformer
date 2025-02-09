use godot::{classes::{CharacterBody2D, ICharacterBody2D}, prelude::*};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    base: Base<CharacterBody2D>
}

const SPEED: f64 = 300.0;
const JUMP_VELOCITY: f64 = -400.0;

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base
        }
    }

    fn physics_process(&mut self, delta: f64) {

    }
}
