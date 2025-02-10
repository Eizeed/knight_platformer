use godot::{classes::{Area2D, IArea2D, PhysicsBody2D, Timer}, prelude::*};

#[derive(GodotClass)]
#[class(base=Area2D)]
struct KillZone {
    base: Base<Area2D>
}

#[godot_api]
impl KillZone {
    #[func]
    fn on_body_enter(&mut self, _body: Gd<PhysicsBody2D>) {
        let mut timer = self.base().get_node_as::<Timer>("Timer");
        timer.start();
        godot_print!("You died");
    }

    #[func]
    fn on_timer_timeout(&mut self) {
        godot_print!("You died");
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
