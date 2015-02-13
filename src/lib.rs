#![crate_id = "event"]
#![crate_type = "rlib"]
#![allow(dead_code)]

use std::rc::Rc;
use std::cmp::Ordering;

#[doc = "Enumerator for Events. This allows you to handle missing Event arguments. Input Args of whatever type you want to Event.Args"]
pub enum Event<E> {
    Args(E),
    Missing,
}

// To deal with handler functions - F: Rc<Box<Fn(&event<E>)>>
#[doc = "EventPublisher. Works similarly to C#'s event publishing pattern. Event handling functions are subscribed to the publisher.
Whenever the publisher fires an event it calls all subscribed event handler functions.

TODO: unsubscribe_handler function to remove handlers from the event publisher."]
pub struct EventPublisher<E> {
    handlers: Vec<Rc<Box<Fn(&Event<E>) + 'static>>>,
}

impl<E> EventPublisher<E> {

    #[doc = "Event publisher constructor."]
    pub fn new() -> EventPublisher<E> {
        EventPublisher{ 
            handlers: Vec::<Rc<Box<Fn(&Event<E>) + 'static>>>::new() 
        }
    }
    #[doc = "Subscribes event handler functions to the EventPublisher.
    
    INPUT: handler_box: Box<Fn(&Event<E>) + 'static>>   handler_box is a box pointer to a function to handle an event of the type E. The function must
        be capable of handling references to the event type set up by the publisher, rather than the raw event itself.
    OUTPUT: void"]
    pub fn subscribe_handler(&mut self, handler_box: Box<Fn(&Event<E>) + 'static>){
        self.handlers.push( Rc::new(handler_box) );
        self.handlers.sort_by(|a,b| (&**a as *const _).cmp(&(&**b as *const _))) 
    }
    /// - Implement this when I've got the rest of it working. Use raw pointer comparison and a binary search
    /*pub fn remove_handler<E,F>(&mut self, handler: Box<F>){
        
    }*/
    
    // TODO: Implement this concurrently
    #[doc = "Publish events. Currently implemented with a simple iterator, but will have multithreading support added later."]
    pub fn publish_event(&self, event: &Event<E>){
        for handler in self.handlers.iter(){
            handler(event);
        }
    }
}