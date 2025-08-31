use std::f32::consts::PI;

use mlua::{Lua, Table};

use crate::shroud_layer_container::ShroudLayerContainer;

pub fn parse_shroud_text(shroud_text: &str) -> Result<Vec<ShroudLayerContainer>, String> {
    let lua = Lua::new();
    lua.load(format!("pi={}", PI)).exec().unwrap();
    if let Err(err) = lua.load(shroud_text).exec() {
        return Err(format!("Failed to parse shroud: {}", err));
    }
    // let shroud: Vec<Table> = lua
    //     .load(shroud_text)
    //     .eval::<Table>()
    //     .unwrap()
    //     .sequence_values::<Table>()
    //     .map(|shroud_layer| shroud_layer.unwrap())
    //     .collect();
    todo!();
}
