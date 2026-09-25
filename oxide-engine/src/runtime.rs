use mlua::Lua;

#[derive(Debug)]
pub struct Runtime {
  lua_vm: Lua
}

impl Runtime {
  pub fn new() -> Self {
    let lua_vm = Lua::new();

    Runtime {
      lua_vm
    }
  }
}



