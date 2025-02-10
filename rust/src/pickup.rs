use godot::{
    classes::{
        AnimatedSprite2D, Area2D, AudioStreamPlayer2D, CollisionShape2D, IArea2D, PhysicsBody2D,
        Timer,
    },
    obj::WithBaseField,
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Coin {
    base: Base<Area2D>,
}

#[godot_api]
impl Coin {
    #[func]
    fn on_body_enter(&mut self, _body: Gd<PhysicsBody2D>) {
        self.base()
            .get_node_as::<AudioStreamPlayer2D>("PickupSound")
            .play();

        self.base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D")
            .set_visible(false);

        self.base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D")
            .call_deferred("set_disabled", &[true.to_variant()]);

        self.base().get_node_as::<Timer>("Timer").start();
    }
}

#[godot_api]
impl IArea2D for Coin {
    fn init(base: Base<Self::Base>) -> Self {
        Self { base }
    }
}
