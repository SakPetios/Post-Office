/*
^ This File Contains All The Misc DataTypes And Traits
*/

use std::collections::HashMap;

use cursive::Cursive;

pub trait State {
    fn render(&mut self, cur: &mut Cursive);
}

#[derive(Clone, Debug)]
pub struct LuaResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl rlua::UserData for LuaResponse {
    fn add_methods<'lua, T: rlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_method("status", |_,this,_:()| {
            Ok(this.status)
        });
        methods.add_method("body", |_,this,_:()| {
            Ok(this.body.clone())
        });
        methods.add_method("header", |_,this,_:()| {
            Ok(this.headers.clone())
        });
    }
}

impl From<reqwest::blocking::Response> for LuaResponse {
    fn from(value: reqwest::blocking::Response) -> Self {
        let headrs = value.headers();
        let header_hash_map: HashMap<String,String> = headrs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string())).collect();
        let resp = LuaResponse {
            status: value.status().as_u16(),
            headers:header_hash_map,
            body:value.text().unwrap()
        };
        resp
    }
}
