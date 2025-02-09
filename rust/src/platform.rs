use godot::{classes::{AnimatableBody2D, IAnimatableBody2D}, prelude::*};

#[derive(GodotClass)]
#[class(base=AnimatableBody2D)]
struct Platform {
    base: Base<AnimatableBody2D>
}

#[godot_api]
impl IAnimatableBody2D for Platform {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base
        }
    }
}
