use crate::creation::components::{PartPosition, Element};
use assets::GameAssets;
use bevy::{prelude::*, utils::HashMap};

pub(crate) mod assets;

#[derive(Clone, Component)]
pub struct BSInspect;

#[derive(Debug)]
pub(crate) struct BaseStorage {
    /// TODO can also be HashMap
    pub elems: [u32; 4],
    /// tuple with elem index and cost in u32
    pub parts: HashMap<PartPosition, (usize, u32)>,
}
impl Default for BaseStorage {
    fn default() -> Self {
        Self {
            elems: [32; 4],
            parts: HashMap::from([
                (PartPosition::Head, (0, 4)),
                (PartPosition::Body, (1, 4)),
                (PartPosition::Legs, (2, 4)),
            ]),
        }
    }
}
impl BaseStorage {
    pub fn escrow(&self) -> [u32; 4] {
        let mut escrow = [0; 4];
        for &(elem, cost) in self.parts.values() {
            escrow[elem] += cost;
        }
        escrow
    }
    pub fn to_str(&self) -> String {
        let escrow = self.escrow();
        format!(
            "Mud: {}{}\nWater: {}{}\nFire: {}{}\nElectric: {}{},",
            self.elems[0],
            if escrow[0] > 0 {
                format!(" - {}", escrow[0])
            } else {
                "".to_string()
            },
            self.elems[1],
            if escrow[1] > 0 {
                format!(" - {}", escrow[1])
            } else {
                "".to_string()
            },
            self.elems[2],
            if escrow[2] > 0 {
                format!(" - {}", escrow[2])
            } else {
                "".to_string()
            },
            self.elems[3],
            if escrow[3] > 0 {
                format!(" - {}", escrow[3])
            } else {
                "".to_string()
            },
        )
    }
    pub fn can_consume_escrow(&self) -> bool {
        let escrow = self.escrow();
        for (i, &item) in escrow.iter().enumerate() {
            if item > self.elems[i] {
                return false;
            }
        }
        true
    }
    pub fn consume_escrow(&mut self) {
        assert!(self.can_consume_escrow());
        for &(elem, cost) in self.parts.values() {
            self.elems[elem] -= cost;
        }
    }
    pub fn change_escrow(&mut self, part: &PartPosition, inc: Option<bool>) {
        if let Some(inc) = inc {
            // hack: need a better check to see if enough elems are present
            let mut limit = 4;
            let escrow = self.escrow();
            let part = self.parts.get_mut(part).unwrap();

            while limit > 0 {
                part.0 = (part.0 + if inc { 1 } else { 3 }) % 4;
                if self.elems[part.0].saturating_sub(escrow[part.0]) > 0 {
                    break;
                }
                limit -= 1;
            }
        }
    }
    /// Spawns a board with the current resource storage
    /// TODO can become a small nodebundle of its own with children nodes for each resource
    /// for now it is only string
    pub fn spawn_board(&self, parent: &mut ChildBuilder, assets: &GameAssets) {
        parent
            .spawn_bundle(NodeBundle::default())
            .with_children(|parent: &mut ChildBuilder| {
                for (elem, value) in self.elems.iter().enumerate() {
                    let elem_name: Element = elem.into();
                    let elem_sm = assets.elem_sprite_material(elem);
                    parent.spawn_bundle(elem_sm.node(Style::default())).with_children(|parent: &mut ChildBuilder|{
                        parent.spawn_bundle(assets.create_text(format!("{elem_name:?}\n{value}")));
                    });
                }
                // parent.spawn_bundle(assets.create_text(&*self.to_str()));
            }).insert(BSInspect);
    }
}

pub(crate) fn base_storage_inspect(
    bs: Res<BaseStorage>,
    children: Query<&Children>,
    board: Query<Entity, With<BSInspect>>,
    mut query: Query<&mut Text>,
) {
    let elem_nodes = children.get(board.get_single().unwrap()).unwrap();
    let escrow = bs.escrow();
    for (elem, &node) in elem_nodes.iter().enumerate() {
        let elem_name: Element = elem.into();
        if let Ok(mut text) = query.get_mut(children.get(node).unwrap()[0]){
            text.sections[0].value = format!("{elem_name:?}\n{}{}",bs.elems[elem],
            if escrow[elem] > 0 {
                format!(" - {}", escrow[elem])
            } else {
                "".to_string()
            },
                                             )
        }
    }
}
