use std::rc::Rc;
use std::cmp::Ordering;

enum Event<E> {
    Args(E),
    Missing,
}

// To deal with handler functions - F: Rc<Box<Fn(&event<E>)>>
struct EventPublisher<E> {
    handlers: Vec<Rc<Box<Fn(&Event<E>) + 'static>>>,
}

impl<E> EventPublisher<E> {

    pub fn new() -> EventPublisher<E> {
        EventPublisher{ 
            handlers: Vec::<Rc<Box<Fn(&Event<E>) + 'static>>>::new() 
        }
    }

    pub fn subscribe_handler(&mut self, handler_rc: Rc<Box<Fn(&Event<E>) + 'static>>){
        self.handlers.push(handler_rc.clone());
        self.handlers.sort_by(|a,b| (&**a as *const _).cmp(&(&**b as *const _)))
    }
    /// - Implement this when I've got the rest of it working. Use raw pointer comparison and a binary search
    /*pub fn remove_handler<E,F>(&mut self, handler: Box<F>){
        
    }*/
    
    pub fn publish_event(&self, event: &Event<E>){
        for handler in self.handlers.iter(){
            handler(event);
        }
    }
}

#[test]
fn it_works() {
}
