/*
^ Unit Tests
*/

#[cfg(test)]
mod lua_tests {
    use crate::backend::LuaBackend;
    #[test]
    fn simple_evaluation() {
        let mut lua = LuaBackend::new();
        assert!(lua.run("1 == 1".to_string()).unwrap())
    }
    #[test]
    fn false_evaluation() {
        let mut lua = LuaBackend::new();
        assert!(!lua.run("1 == 2".to_string()).unwrap())
    }
    #[test]
    fn error_evaluation() {
        let mut lua = LuaBackend::new();
        assert!(lua.run("1 =sdfg= 1".to_string()).is_err())
    }
}

// #[cfg(test)]
// mod manager_tests {
//     use crate::manager::Manager;
//     #[test]
//     fn init() {
//         let mng = Manager::new("tests".to_string());
//         assert!(mng.is_ok())
//     }
//     #[test]
//     fn load() {
//         let mut mng = Manager::new("tests".to_string()).unwrap();
//         let ls = mng.list_lua();
//         assert!(ls.is_ok());
//         assert!(!ls.unwrap().is_empty());
//     }
// }
