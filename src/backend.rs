
use log::{debug, error};
use rlua::Lua;

#[derive(Debug)]
pub enum LuaError {
    SyntaxError
}

pub struct LuaBackend {
    luac: Lua,
}

impl LuaBackend {
    pub fn new() -> Self {
        LuaBackend { luac: Lua::new() }
    }

    pub fn init(&mut self) {
        
    } // TODO Add functions
    
    /// **Runs One Test Expect A Bool From It**
    pub fn run(&mut self, code: String) -> Result<bool,LuaError> {
        let out = self.luac.context(|ctx| ctx.load(&code).eval::<bool>());

        let result = match out {
            Ok(bol) => bol,
            Err(er) => {
                error!("Error Running Lua code");
                debug!("Error message: {}", er);

                return Err(LuaError::SyntaxError);
            }
        };
        Ok(result)
    }
}
