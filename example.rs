use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct GameContext {
    // shared game data goes here
    shared_game_data_etc: (),
}

impl GameContext {
    pub fn const_method(&self) {
        println!("Called GameContext.const_method()")
    }

    pub fn mut_method(&mut self) {
        println!("Called GameContext.mut_method()")
    }
}

// Wrapping the contex into an Rc allows you to store it multiple places
// This is like shared_ptr in C++, but not thread-safe (use Arc<Mutex<> if you need thread safety).
type Context = Rc<RefCell<GameContext>>;

pub trait Update {
    fn update<'a>(&mut self, context: Context, chunk: &'a mut Chunk);

    // You could pass the context by reference here as well, but then you won't be able to store it
    // in multiple places at the same time. Maybe that's fine, so see for yourself.
    // fn update<'a, 'ctx>(&mut self, context: &'ctx mut Context, &'a mut Chunk);
}

pub struct Chunk {
    // You might want to allocate these in an 'arena' if they're short-lived (e.g. dropped/replaced after every frame/couple of frames)
    dynamic: Vec<Box<dyn Update>>,
    statics: Vec<() /* pretend that this is your static enum */>,
}

impl Chunk {
    fn update(&mut self, context: Context) {
        // Move the dynamic objects into a local variable
        // (self.dynamic is now an empty vec)
        let mut dynamic = std::mem::take(&mut self.dynamic);
        for obj in &mut dynamic {
            // Calling clone() here, because it's cheap
            obj.update(context.clone(), self);
        }
        self.dynamic = dynamic;

        for _ in &self.statics {
            // etc
        }
    }

    pub fn some_chunk_method(&self, s: &str, x: usize) {
        println!("{} {}", s, x);
    }
}

struct DynamicThing;
struct UpdateCounter(usize);

impl Update for DynamicThing {
    fn update<'a>(&mut self, context: Context, _chunk: &'a mut Chunk) {
        let const_ctx_ref = context.borrow();
        println!(
            "ctx.shared_game_data_etc = {:?}",
            const_ctx_ref.shared_game_data_etc
        );
        const_ctx_ref.const_method();

        // have to drop the contst reference before we can borrow_mut
        std::mem::drop(const_ctx_ref);

        let mut mut_ctx_ref = context.borrow_mut();
        mut_ctx_ref.mut_method();
    }
}

impl Update for UpdateCounter {
    fn update<'a>(&mut self, _context: Context, chunk: &'a mut Chunk) {
        self.0 += 1;
        chunk.some_chunk_method("Updated the chunk this many times:", self.0);
    }
}

fn main() {
    let context = GameContext::default();
    let shared_context = Rc::new(RefCell::new(context));

    let mut chunk = Chunk {
        dynamic: vec![],
        statics: vec![],
    };
    chunk.dynamic.push(Box::new(DynamicThing));
    chunk.dynamic.push(Box::new(UpdateCounter(0)));

    for _ in 0..10 {
        chunk.update(shared_context.clone());
        println!("");
    }
}

/////////////////////////////////////////////////////////////////////////////////////
///
///
///
#[allow(unused)]
trait Update {
    fn update(&mut self, chunk: &Chunk);
}

enum Dynamic {
    Empty,
    Value(Box<dyn Update>),
}

struct Chunk {
    dynamics: Vec<Dynamic>,
}

impl Chunk {
    pub fn update(&mut self) {
        for i in 0..self.dynamics.len() {
            let mut dynamic = std::mem::replace(self.dynamics.get_mut(i).unwrap(), Dynamic::Empty);
            match &mut dynamic {
                Dynamic::Value(obj) => obj.update(self),
                _ => (),
            }
            self.dynamics[i] = dynamic;
        }
    }
}
