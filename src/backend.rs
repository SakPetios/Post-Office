use log::{debug, error};
use rlua::{Context, Lua};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
const SIGNATURE: &str = "reportOutPut";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Data {
    TestResult { name: String, result: bool },
}

pub struct LuaBackend {
    luac: Lua,
}

impl LuaBackend {
    pub fn new() -> Self {
        LuaBackend { luac: Lua::new() }
    }

    pub fn init(&mut self) {
        self.luac.context(|ctx| {
            let globals = ctx.globals();

            // + Init Test Results Variable
            let udata = Vec::<Data>::new();
            let jsondata = match serde_json::to_string(&udata) {
                Ok(dt) => dt,
                Err(er) => {
                    log::info!("Error Serializing UserData");
                    log::error!("Error Message: {}", er);
                    return;
                }
            };
            globals.set(SIGNATURE, jsondata).unwrap();
        });
        // + Create Functions
        
        self.create_testfl();
    } // TODO Add functions
    #[allow(unused)]
    /// **Runs One Test Expect A Bool From It**
    pub fn run(&mut self, code: String) -> Result<bool, rlua::Error> {
        let out = self.luac.context(|ctx| ctx.load(&code).eval::<bool>());

        let result = match out {
            Ok(bol) => bol,
            Err(er) => {
                error!("Error Running Lua code");
                debug!("Error message: {}", er);

                return Err(er);
            }
        };
        Ok(result)
    }
    pub fn blueprint(&mut self, code: String) -> Result<(), rlua::prelude::LuaError> {
        self.luac.context(|ctx| ctx.load(&code).exec())
    }
    /// Fetches Result Test Data
    pub fn fetch(&mut self) -> Vec<Data> {
        self.luac.context(|ctx| {
            let globs = ctx.globals();
            let raw_data: String = globs.get(SIGNATURE).unwrap();
            log::debug!("{}", raw_data);
            serde_json::from_str(&raw_data).unwrap()
        })
    }
    fn create_testfl(&self) {
        self.luac.context(|ctx| {
            let globals = ctx.globals();
            let test_file = ctx
                .create_function(|ctx, file: String| {
                    // + Read File
                    // ^ If file does not exist or is not file Throws Lua Runtime Error
                    let path = Path::new(&file);
                    if !path.exists() {
                        return Err(rlua::Error::RuntimeError("File Not Found".to_string()));
                    }
                    if !path.is_file() {
                        return Err(rlua::Error::RuntimeError("Not File".to_string()));
                    }
                    let content = fs::read_to_string(path);
                    let content = match content {
                        Ok(cont) => cont,
                        Err(_er) => {
                            return Err(rlua::Error::RuntimeError(
                                "Unable To Read File".to_string(),
                            ))
                        }
                    };

                    // + Update UserData
                    let test_result = ctx.load(&content).eval::<bool>().unwrap();
                    let result = Data::TestResult {
                        name: file,
                        result: test_result,
                    };

                    update_report(&ctx, result);

                    Ok(())
                })
                .unwrap();
            globals.set("testfl", test_file).unwrap();
        });
    }
}

/// Updates The SINGATURE variable
fn update_report(ctx: &Context, val: Data) {
    let globs = ctx.globals();

    let raw_data: String = globs.get(SIGNATURE).unwrap();
    log::debug!("{}", raw_data);
    let mut usrdata = serde_json::from_str::<Vec<Data>>(&raw_data).unwrap();
    usrdata.push(val);

    let json_data = serde_json::to_string(&usrdata);
    globs.set(SIGNATURE, json_data.unwrap()).unwrap();
}
