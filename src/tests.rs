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

