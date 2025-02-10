use godot::{
    obj::{NewGd, WithBaseField},
    prelude::*,
};

use crate::{gamemanager, pickup};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Game {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Game {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
        }
    }

    fn ready(&mut self) {
        let gamemanager = self
            .base()
            .get_node_as::<gamemanager::GameManager>("GameManager");

        let coins = self.base().get_node_as::<Node>("Coins").get_children();
        coins.iter_shared().for_each(|child| {
            let mut coin = child.cast::<pickup::Coin>();
            coin.connect("body_entered", &gamemanager.callable("add_point"));
        });
    }
}
