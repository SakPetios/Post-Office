use cursive::Cursive;

pub trait State {
    fn render(&mut self,cur: &mut Cursive);
}