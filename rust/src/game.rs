use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Game {
    base: Base<Node2D>
}

#[godot_api]
impl INode2D for Game {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base
        }
    }
}
