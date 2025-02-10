use godot::{classes::{AnimatedSprite2D, RayCast2D}, obj::WithBaseField, prelude::*};

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Slime {
    direction: f32,

    base: Base<Node2D>
}

const SPEED: f32 = 60.0;

#[godot_api]
impl INode2D for Slime {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            direction: 1.0,
            base
        }
    }

    fn process(&mut self, delta: f64) {
        let ray_cast_right = self.base().get_node_as::<RayCast2D>("RayCastRight");
        let ray_cast_left = self.base().get_node_as::<RayCast2D>("RayCastLeft");

        let mut animation = self.base().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        if ray_cast_right.is_colliding() {
            self.direction = -1.0;
            animation.set_flip_h(true);
            
        }
        if ray_cast_left.is_colliding() {
            self.direction = 1.0;
            animation.set_flip_h(false);
        }

        animation.play();

        let mut position = self.base().get_position();


        position.x += self.direction * SPEED * delta as f32;
        self.base_mut().set_position(position);
    }
}
