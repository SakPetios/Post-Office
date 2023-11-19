use crate::colls::{self, LuaResponse};
use log::{debug, error};
use rlua::{Context, Lua};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};

const SIGNATURE: &str = "reportOutPut";
const BUFFERNAME: &str = "stdBuffer";

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
            let jsondata = serde_json::to_string(&udata).unwrap();
            globals.set(SIGNATURE, jsondata).unwrap();

            // + Create The StdOut Buffer
            let buff = Vec::<String>::new();
            let jsonbuff = serde_json::to_string(&buff).unwrap();
            globals.set(BUFFERNAME, jsonbuff).unwrap();
        });

        // + Create Functions
        self.create_testfl();
        self.create_get();
        self.create_stdout();
        self.create_assert();
        self.create_post();
    }
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
    /// Fetches Standart Output
    pub fn fetch_stdout(&mut self) -> Result<Vec<String>, ()> {
        // TODO Add Errors
        self.luac.context(|ctx| {
            let globals = ctx.globals();
            let raw_data: String = match globals.get(BUFFERNAME) {
                Ok(data) => data,
                Err(er) => {
                    log::error!("Error Getting Buffer");
                    log::debug!("Error message: {}", er);
                    return Err(());
                }
            };
            match serde_json::from_str(&raw_data) {
                Ok(buff) => Ok(buff),
                Err(_er) => Err(()),
            }
        })
    }
    /// Creates the post method for Lua
    ///
    /// **Arguments**
    /// - URL
    /// - body
    /// - form
    /// - headers
    fn create_post(&self) {
        // TODO Add Headers
        self.luac.context(|ctx| {
            let post_fn = ctx.create_function(
                |_,
                 (url, body, form, headers): (
                    String,
                    Option<String>,
                    Option<HashMap<String, String>>,
                    Option<HashMap<String, String>>,
                )| {
                    // + Create Client & Request
                    let cli = reqwest::blocking::Client::new();
                    let mut req = cli.post(&url);
                    if let Some(bdy) = &body {
                        req = req.body(bdy.clone());
                    };
                    if let Some(frm) = &form {
                        req = req.form(frm);
                    };
                    if let Some(hdrs) = &headers {
                        for (key,value) in hdrs {
                            req = req.header(key, value)
                        }
                    }
                    log::info!(
                        "Sending Post Request at: {} with body: {:?}, form: {:?} and headers: {:?}",
                        url,
                        body,
                        form,
                        headers
                    );
                    let response = match req.send() {
                        Ok(resp) => resp,
                        Err(er) => {
                            log::error!("Error sending request at {}", &url);
                            log::debug!("Error message: {}", er);
                            log::debug!("Function parameters: \nurl: {},body: {:?}", &url, &body);
                            return Err(rlua::Error::RuntimeError(format!(
                                "Error Sending Request at: {}, body: {:?}",
                                &url,
                                &if let Some(bd) = &body {
                                    &bd[..30]
                                } else {
                                    "No body included"
                                }
                            )));
                        }
                    };
                    let resp = LuaResponse::from(response);
                    Ok(resp)
                },
            );
            let globals = ctx.globals();
            match globals.set("post", post_fn.unwrap()) {
                Ok(_) => (),
                Err(er) => {
                    log::error!("Error setting post function");
                    log::debug!("Error message: {}", er);
                }
            };
        })
    }
    /// **Arguments**
    /// - URL
    /// - Headers
    /// - Query Strings
    fn create_get(&self) {
        self.luac.context(|ctx| {
            let globals = ctx.globals();
            let get_method = ctx
                .create_function(
                    |_,
                     (url, hdrs, queries): (
                        String,
                        Option<HashMap<String, String>>,
                        Option<HashMap<String, String>>,
                    )| {
                        log::info!(
                            "Sending GET request at: {} with {:?} headers and {:?} queries",
                            url,
                            hdrs,
                            queries
                        );
                        // + Create Client - Send Request
                        let client = reqwest::blocking::Client::new();
                        let mut req = client.get(&url);
                        if let Some(headrs) = hdrs {
                            for (key, value) in headrs {
                                req = req.header(key, value);
                            }
                        }
                        if let Some(queries) = queries {
                            for (key, value) in queries {
                                req = req.query(&[(key, value)])
                            }
                        }

                        let response = match req.send() {
                            Ok(rs) => rs,
                            Err(er) => {
                                log::error!("Error Sending Request");
                                log::debug!("Error Message: {}", er);
                                return Err(rlua::Error::RuntimeError(format!(
                                    "Error Sending Request\n{}",
                                    er
                                )));
                            }
                        };
                        let lua_response = colls::LuaResponse::from(response);
                        Ok(lua_response)
                    },
                )
                .unwrap();
            globals.set("get", get_method).unwrap();
        });
    }
    /// **Arguments**
    /// - Name: Name Of The Test
    /// - Result (boolean)
    fn create_assert(&self) {
        self.luac.context(|ctx| {
            let assert = ctx.create_function(|ctx, (name, result): (String, bool)| {
                log::debug!("Running Assert");
                update_report(&ctx, Data::TestResult { name, result });
                Ok(())
            });
            let globals = ctx.globals();
            match globals.set("rassert", assert.unwrap()) {
                // + Name difficulties
                Ok(_) => {
                    log::debug!("Succefully initialized assert function")
                }
                Err(er) => {
                    log::error!("Error Creating Assert");
                    log::debug!("Error message: {}", er);
                    return;
                }
            };
        })
    }
    /// Reads Lua File and Executes it. Expects Boolean
    /// **Arguments**
    /// - File
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
                    let test_result = match ctx.load(&content).eval::<bool>() {
                        Ok(res) => res,
                        Err(er) => {
                            log::error!("Error Running Testfl");
                            log::debug!("Error Message: {}", er);
                            return Err(rlua::Error::RuntimeError(format!(
                                "Error Running testfl\n{}",
                                er
                            )));
                        }
                    };
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
    /// Works like a print function
    /// the stdout is shown in the LuaConsole
    /// instead of the console
    /// **Arguments**
    /// - Text
    fn create_stdout(&mut self) {
        self.luac.context(|ctx| {
            let stdout_func = ctx
                .create_function(|ctx, stri: String| {
                    let globals = ctx.globals();
                    let rawbuff: String = globals.get(BUFFERNAME).unwrap();
                    let mut buffer: Vec<String> = match serde_json::from_str(&rawbuff) {
                        Ok(buff) => buff,
                        Err(er) => {
                            log::error!("Error Deserizlizing Buffer");
                            log::debug!("Error message: {}", er);
                            return Err(rlua::Error::SyntaxError {
                                message: format!("Unable To Deserialize Buffer\n{}", er),
                                incomplete_input: false,
                            });
                        }
                    };
                    buffer.push(stri);
                    let json_buff = match serde_json::to_string(&buffer) {
                        Ok(buff) => buff,
                        Err(er) => {
                            log::error!("Error Serizlizing Buffer");
                            log::debug!("Error message: {}", er);
                            return Err(rlua::Error::SyntaxError {
                                message: format!("Unable To Serialize Buffer\n{}", er),
                                incomplete_input: false,
                            });
                        }
                    };
                    match globals.set(BUFFERNAME, json_buff) {
                        Ok(_) => (),
                        Err(er) => {
                            log::error!("Error Setting Buffer");
                            log::debug!("Error Message: {}", er);
                            return Err(er);
                        }
                    };
                    Ok(())
                })
                .unwrap();
            let globals = ctx.globals();
            match globals.set("stdout", stdout_func) {
                Ok(_) => (),
                Err(er) => {
                    log::error!("Error Setting stdout variable");
                    log::debug!("Error Message:\n{}", er);
                    return;
                }
            };
        })
    }
}

/// Updates The SINGATURE variable
/// Really Importand Function.
/// This function writes in the SIGNATURE variable the val after seriallizing it to json
/// The Signature variable holds the test results (Data). It can Also be used for reporting Results
fn update_report(ctx: &Context, val: Data) {
    let globs = ctx.globals();

    let raw_data: String = globs.get(SIGNATURE).unwrap();
    log::debug!("{}", raw_data);
    let mut usrdata = serde_json::from_str::<Vec<Data>>(&raw_data).unwrap();
    usrdata.push(val);

    let json_data = serde_json::to_string(&usrdata);
    globs.set(SIGNATURE, json_data.unwrap()).unwrap();
}
