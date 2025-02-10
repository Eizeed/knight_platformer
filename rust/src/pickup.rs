use godot::{classes::{Area2D, IArea2D, PhysicsBody2D}, obj::WithBaseField, prelude::*};

#[derive(GodotClass)]
#[class(base=Area2D)]
struct Coin {
    base: Base<Area2D>
}

#[godot_api]
impl Coin {
    #[func]
    fn on_body_enter(&mut self, _body: Gd<PhysicsBody2D>) {
        self.base_mut().queue_free();
        godot_print!("+1")
    }
}

#[godot_api]
impl IArea2D for Coin {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base
        }
    }
}
