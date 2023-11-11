/*
^ This File Contains All The Misc DataTypes And Traits
*/

use cursive::Cursive;

pub trait State {
    fn render(&mut self,cur: &mut Cursive);
}