#![allow(dead_code)]

use std::collections::BTreeMap;
use std::sync::Arc;
use std::thread::{Thread, JoinGuard};

/// Enumerator of the Event type. Whatever type E of Event::Args you implement here is the type E that will be used for the EventPublisher.
pub enum Event<E: Send + Sync> {
    Args(E),
    Missing,
}

// To deal with handler functions - F: Rc<Box<Fn(&event<E: Send + Sync>)>>
/// EventPublisher. Works similarly to C#'s event publishing pattern. Event handling functions are subscribed to the publisher.
/// Whenever the publisher fires an event it calls all subscribed event handler functions.
/// Use event::EventPublisher::<E: Send + Sync>::new() to construct
pub struct EventPublisher<E: Send + Sync> {
    handlers: BTreeMap<usize, Arc<Box<Fn(&Event<E>) + Send + Sync>>>,
}

impl<E> EventPublisher<E> where E: Send + Sync{

    /// Event publisher constructor.
    pub fn new() -> EventPublisher<E> {
        EventPublisher{ 
            handlers: BTreeMap::<usize, Arc<Box<Fn(&Event<E>) + Send + Sync>>>::new() 
        }
    }
    /// Subscribes event handler functions to the EventPublisher.
    /// INPUT:  handler_box: Box<Fn(&Event<E: Send + Sync>) + Send + Sync>>   handler_box is a box pointer to a function to handle an event of the type E. The function must
    ///     be capable of handling references to the event type set up by the publisher, rather than the raw event itself.
    /// OUTPUT: void
    pub fn subscribe_handler(&mut self, handler_box: Box<Fn(&Event<E>) + Send + Sync>){

        let p_handler = &*handler_box as *const _;
        self.handlers.insert(p_handler as usize, Arc::new(handler_box));
    }
    
    /// Unsubscribes an event handler from the publisher.
    /// INPUT:  handler_box: Box<Fn(&Event<E: Send + Sync>) + Send + Sync>    handler_box is a box pointer to a function to handle an event of the type E.
    /// OUTPUT: bool    output is a bool of whether or not the function was found in the list of subscribed event handlers and subsequently removed.
    pub fn unsubscribe_handler(&mut self, handler_box: Box<Fn(&Event<E>) + Send + Sync>) -> bool {
    
        let p_handler = &*handler_box as *const _;
        match self.handlers.remove(&(p_handler as usize)){
        Some(_) => true,
        None => false,
        }       
    }
    
    // TODO: Implement this concurrently
    /// Publishes events, pushing the &Event<E: Send + Sync> to all handler functions stored by the event publisher.
    /// INPUT: event: &Event<E: Send + Sync>     Reference to the Event<E: Send + Sync> being pushed to all handling functions.
    pub fn publish_event(&self, event: &Event<E>){
        for (_,handler) in self.handlers.iter(){
            handler(event);
        }
    }
    
    pub fn publish_event_multithreaded(&self, event: &mut Event<E>){
        let p_event = std::ptr::Unique(event);
        let shared_event = Arc::new(p_event);
        let mut guards = Vec::<JoinGuard<_>>::new();
        
        for (_,handler) in self.handlers.iter(){
            //let cloned_handler = handler.clone();
            let cloned_event = shared_event.clone();
            unsafe {
                guards.push(Thread::scoped(move || {
                    let moved_event = cloned_event;
                    handler(&(*moved_event.ptr))}));
            }
        }
    }
}