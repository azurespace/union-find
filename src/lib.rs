use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Clone)]
enum SetItem<ValueType> {
    Root(ValueType, u32),
    Link(Box<SetItem<ValueType>>)
}

/* The structure saves the parent information of each subset in continuous 
 * memory(a vec) for better performance.
 *
 * Each T entry is mapped onto a usize tag.
 */
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
        match self.map.get(&x) {
            Some(p) => return,
            None => {}
        }

        self.map.insert(x, *len);
        self.parent.push(*len);

        *len += 1;
    }

    /* Returns Some(num), num is the tag of subset in which x is.
     * If x is not in the data structure, it returns None.
     * 
     */
    fn find(&mut self, x: T) -> Option<usize> 
    {
        let mut pos: usize;
        match self.map.get(&x) {
            Some(p) => {pos = *p;},
            None => return None
        }

        let ret = DisjointSet::<T>::find_internal(&mut self.parent, pos);
        Some(ret)            
    }

    fn find_internal(p: &mut Vec<usize>, n: usize) -> usize{
        if p[n] != n{
            let parent = p[n];
            p[n] = DisjointSet::<T>::find_internal(p, parent);
            p[n]
        }
        else {
            n
        }
    }

    /* Union the subsets to which x and y belong .
     * 
     */
    fn union(&mut self, x: T, y: T) -> Result<usize, ()>
    {
        let x_root;
        let y_root;
        match self.find(x) {
            Some(x_r) => {x_root = x_r;} ,
            None => {return Err(());}
        }

        match self.find(y) {
            Some(y_r) => {y_root = y_r;} ,
            None => {return Err(());}
        }

        self.parent[x_root] = y_root;
        Ok(y_root)
    }
}

#[test]
fn it_works() {
    let mut ds = DisjointSet::<i32>::new();
    ds.make_set(1);
    ds.make_set(2);
    ds.make_set(3);

    assert!(ds.find(1) != ds.find(2));
    assert!(ds.find(2) != ds.find(3));
    ds.union(1, 2);
    ds.union(2, 3);
    assert!(ds.find(1) == ds.find(3));

    assert!(ds.find(4) == None);
    ds.make_set(4);
    assert!(ds.find(4) != None);


    ds.make_set(-1);
    assert!(ds.find(-1) != ds.find(3));

    ds.union(-1, 4);
    ds.union(2, 4);

    assert!(ds.find(-1) == ds.find(3));
}



