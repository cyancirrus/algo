use std::cell::RefCell;
use std::rc::Rc;
use std::collections::BTreeMap;

type Capacity = usize;
const ALLOCATED_STACK_MEM:Capacity = 1024;


struct Allocation {
    capacity:Capacity,
    head:usize,
    tail:usize,
}

struct ArenaAllocator<T> {
    mem: [Option<T>;ALLOCATED_STACK_MEM],
    head:usize,
    tail:usize,
    freed:BTreeMap<Capacity, Vec<Allocation>>,
    capacity:usize,
}

type Arena<T> = Rc<RefCell<ArenaAllocator<T>>>;

impl <T> ArenaAllocator<T> 
where T: Copy
{
    fn new() -> Self {
        Self {
            mem:[None;ALLOCATED_STACK_MEM],
            head:0,
            tail:0,
            freed:BTreeMap::new(),
            capacity: ALLOCATED_STACK_MEM,

        }
    }
    fn allocate(&mut self, size:Capacity) -> Result <Allocation, &'static str> {
        if let Some(f) = self.freed.get_mut(&size) {
            if let Some(m) = f.pop() {
                return Ok(m)
            }
        }
        if size < self.capacity - self.tail  {
            let alloc = Allocation {
                capacity:size,
                head:self.tail,
                tail:self.tail + size,
            };
            self.tail += size;
            Ok(alloc)
        } else {
            Err("Memory exceeded either too fragmented or at capacity")
        }
    }
    fn free(&mut self, alloc:Allocation) -> Result <(), &'static str> {
        for v in &mut self.mem[alloc.head..alloc.tail] {
            *v = None;
        };
        Ok(self.freed.entry(alloc.capacity).or_default().push(alloc))
    }
}

struct ArenaInterface<T> {
    allocation:Allocation,
    arena:Arena<T>
}

impl <T> ArenaInterface<T> {
    fn new(arena:&Arena<T>, size:Capacity) -> Result<Self, &'static str> {
        let a = arena.clone();
        match a.borrow_mut().allocate(size) {
            Ok(alloc) => {
                Ok(Self {
                    allocation:alloc,
                    arena:a,
                }
                )
            },
            Err(e) => {
                Err(e)
            }
        }
    }
    fn insert(&mut self, val:T) {
        let a = self.arena.borrow_mut();
        a.mem[self.allocation.tail] = val;
        self.allocation.tail += 1;
    }
    fn view(&self) {
        println!("
            Current Data {:?}",
            &self.arena[self.allocation.head..self.allocation.tail]
        )
    }
}


fn main() {
    let mut arena: Arena<usize> = Rc::new(RefCell::new(ArenaAllocator::new()));
    let xarena = ArenaInterface::new(&arena, 128);
    let yarena = ArenaInterface::new(&arena, 256);


    // xarena.lock().allocate(216);
    // xarena.
}
