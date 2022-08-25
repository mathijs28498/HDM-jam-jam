use bevy::prelude::*;
use assets::GameAssets;

pub(crate) mod assets;

#[derive(Clone, Component)]
pub struct BSInspect;

#[derive(Debug)]
pub(crate) struct BaseStorage {
    pub elems: [u32;4],
    /// tuple with elem index and cost in u32
    pub head: (usize, u32),
    pub body: (usize, u32),
    pub legs: (usize, u32),
}
impl Default for BaseStorage {
    fn default() -> Self {
        Self {
            elems: [32;4],
            head: (0, 4),
            body: (1, 4),
            legs: (2, 4),
        }
    }
}
impl BaseStorage {
    pub fn escrow(&self) -> [u32;4] {
        let mut escrow = [0;4];
        escrow[self.head.0] += self.head.1;
        escrow[self.body.0] += self.body.1;
        escrow[self.legs.0] += self.legs.1;
        escrow
    }
    pub fn to_str(&self) -> String {
        let escrow = self.escrow();
        format!("Mud: {}{}\nWater: {}{}\nFire: {}{}\nElectric: {}{},",
                self.elems[0], if escrow[0] > 0 {format!(" - {}",escrow[0])} else {"".to_string()},
                self.elems[1], if escrow[1] > 0 {format!(" - {}",escrow[1])} else {"".to_string()},
                self.elems[2], if escrow[2] > 0 {format!(" - {}",escrow[2])} else {"".to_string()},
                self.elems[3], if escrow[3] > 0 {format!(" - {}",escrow[3])} else {"".to_string()},
                )
    }
    pub fn can_consume_escrow(&self) -> bool {
        let escrow = self.escrow();
        for i in 0..3 {
            if escrow[i] > self.elems[i] {
                return false
            }
        }
        return true
    }
    pub fn consume_escrow(&mut self) {
        assert!(self.can_consume_escrow());
        self.elems[self.head.0] -= self.head.1;
        self.elems[self.body.0] -= self.body.1;
        self.elems[self.legs.0] -= self.legs.1;
    }
    /// Spawns a board with the current resource storage
    /// TODO can become a small nodebundle of its own with children nodes for each resource
    /// for now it is only string
    pub fn spawn_board(&self,
        mut parent: &mut ChildBuilder,
        assets: &GameAssets,
    ) {
        parent
            .spawn_bundle(assets.create_text(&*self.to_str()))
            .insert(BSInspect);
    }
}

pub(crate) fn base_storage_inspect(bs: Res<BaseStorage>, mut query:Query<&mut Text, With<BSInspect>>){
    let mut text = query.get_single_mut().unwrap();
    text.sections[0].value = bs.to_str();
}
