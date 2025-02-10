use godot::{
    classes::{Label, PhysicsBody2D},
    obj::WithBaseField,
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GameManager {
    score: i64,
    base: Base<Node>,
}

#[godot_api]
impl GameManager {
    #[func]
    pub fn add_point(&mut self, _: Gd<PhysicsBody2D>) {
        let mut score_label = self.base().get_node_as::<Label>("ScoreLabel");

        self.score += 1;
        godot_print!("{}", self.score);

        score_label.set_text(&format!("You collected {} coins.", self.score));
    }
}

#[godot_api]
impl INode for GameManager {
    fn init(base: Base<Self::Base>) -> Self {
        Self { score: 0, base }
    }
}
