use godot::{classes::{Area2D, CollisionShape2D, Engine, IArea2D, PhysicsBody2D, Timer}, prelude::*};

#[derive(GodotClass)]
#[class(base=Area2D)]
struct KillZone {
    base: Base<Area2D>
}

#[godot_api]
impl KillZone {
    #[func]
    fn on_body_enter(&mut self, body: Gd<PhysicsBody2D>) {
        let mut timer = self.base().get_node_as::<Timer>("Timer");
        Engine::singleton().set_time_scale(0.5);
        godot_print!("You died");

        body.get_node_as::<CollisionShape2D>("CollisionShape2D").queue_free();

        timer.start();
    }

    #[func]
    fn on_timer_timeout(&mut self) {
        Engine::singleton().set_time_scale(1.0);
        self.base().get_tree().unwrap().reload_current_scene();
    }
}

#[godot_api]
impl IArea2D for KillZone {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base
        }
    }
}
