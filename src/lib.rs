use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Clone)]
enum SetItem<ValueType> {
    Root(ValueType, u32),
    Link(Box<SetItem<ValueType>>)
}

pub struct DisjointSet<T: Clone>
{
    set_size: usize,
    parent: Vec<usize>,
    map: HashMap<T, usize>
}

impl<T> DisjointSet<T> where T: Clone+Hash+Eq
{
    fn new() -> Self{
        DisjointSet{
            set_size: 0,
            parent: Vec::new(),
            map: HashMap::new()
        }
    }

    fn make_set(&mut self, x: T){
        let mut len = &mut self.set_size;
        self.map.insert(x, *len);
        self.parent.push(*len);

        *len += 1;
    }

    fn find(&mut self, x: T) -> Option<usize> 
    {
        let mut pos: usize;
        match self.map.get(&x) {
            Some(p) => {pos = *p;},
            None => return None
        }

        let ret = find_internal(&mut x.parent, pos);
        Some(ret)
    }
}

#[test]
fn it_works() {
}

fn find_internel(p: &mut Vec<usize>, n: usize) -> usize{
    if p[n] != n{
        p[n] = find_internal(p, p[n])
    }
    else {
        n
    }
}


