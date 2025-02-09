use godot::{
    classes::{CharacterBody2D, ICharacterBody2D, ProjectSettings},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    base: Base<CharacterBody2D>,
}

const SPEED: f64 = 130.0;
const JUMP_VELOCITY: f64 = -300.0;

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<Self::Base>) -> Self {
        Self { base }
    }

    fn physics_process(&mut self, delta: f64) {
        let mut velocity = self.base_mut().get_velocity();

        let input = Input::singleton();

        let gravity: f32 = ProjectSettings::singleton()
            .get_setting("physics/2d/default_gravity")
            .to();

        if !self.base().is_on_floor() {
            velocity.y += gravity * delta as f32;
        }

        if input.is_action_pressed("ui_accept") && self.base().is_on_floor() {
            velocity.y = JUMP_VELOCITY as f32;
        }

        let direction = input.get_axis("ui_left", "ui_right");
        if direction != 0.0 {
            velocity.x = direction * SPEED as f32;
        } else {
            velocity = velocity.move_toward(Vector2::new(0.0, velocity.y), SPEED as f32);
        }
        
        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
    }
}
