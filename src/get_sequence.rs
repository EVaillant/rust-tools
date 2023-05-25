use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::{Rc, Weak};

pub struct Task {
    id: u32,
    childs: Vec<Weak<RefCell<Task>>>,
}

impl Task {
    pub fn new(id: u32) -> Rc<RefCell<Self>> {
        let v = Self {
            id,
            childs: Default::default(),
        };
        Rc::new(RefCell::new(v))
    }

    pub fn add_child(&mut self, child: &Rc<RefCell<Self>>) {
        self.childs.push(Rc::downgrade(child));
    }

    fn get_sequence_(
        &self,
        already: &mut HashSet<u32>,
        path: &mut Vec<u32>,
    ) -> Result<Vec<u32>, String> {
        let mut ret = Vec::new();
        path.push(self.id);
        for child in self.childs.iter() {
            let child = child.upgrade().unwrap();
            let child = child.borrow();
            if already.insert(child.id) {
                let mut sub_ret = child.get_sequence_(already, path)?;
                ret.append(&mut sub_ret);
            } else if path.contains(&child.id) {
                return Err(format!("cycle {:?}", path));
            }
        }
        path.pop();
        ret.push(self.id);
        Ok(ret)
    }

    pub fn get_sequence(&self) -> Result<Vec<u32>, String> {
        let mut already = HashSet::new();
        already.insert(self.id);
        self.get_sequence_(&mut already, &mut Vec::new())
    }
}
