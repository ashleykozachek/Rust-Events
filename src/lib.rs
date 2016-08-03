#![allow(dead_code)]

use std::collections::BTreeMap;
use std::thread;
use std::clone;

extern crate num_cpus;

/// Enumerator of the Event type. Whatever type E of Event::Args you implement here is the type E that will be used for the EventPublisher.
pub enum Event<E: Clone + Sync> {
    Args(E),
    Missing,
}

// To deal with handler functions - F: Rc<Box<Fn(&event<E: Send + Sync>)>>
/// EventPublisher. Works similarly to C#'s event publishing pattern. Event handling functions are subscribed to the publisher.
/// Whenever the publisher fires an event it calls all subscribed event handler functions.
/// Use event::EventPublisher::<E: Send + Sync>::new() to construct
pub struct EventPublisher<E: Clone + Sync> {
    handlers: BTreeMap<usize, fn(&Event<E>)>,
}

impl<E> EventPublisher<E> where E: Clone + Sync {

    /// Event publisher constructor.
    pub fn new() -> EventPublisher<E> {
        EventPublisher{ 
            handlers: BTreeMap::<usize, fn(&Event<E>)>::new() 
        }
    }
    /// Subscribes event handler functions to the EventPublisher.
    /// INPUT:  handler_box: Box<Fn(&Event<E: Send + Sync>) + Send + Sync>>   handler_box is a box pointer to a function to handle an event of the type E. The function must
    ///     be capable of handling references to the event type set up by the publisher, rather than the raw event itself.
    /// OUTPUT: void
    pub fn subscribe_handler(&mut self, handler: fn(&Event<E>)){
        let p_handler: usize;
        unsafe{
            p_handler = *(handler as *const usize);
        }
        self.handlers.insert(p_handler, handler);
    }
    
    /// Unsubscribes an event handler from the publisher.
    /// INPUT:  handler_box: Box<Fn(&Event<E: Send + Sync>) + Send + Sync>    handler_box is a box pointer to a function to handle an event of the type E.
    /// OUTPUT: bool    output is a bool of whether or not the function was found in the list of subscribed event handlers and subsequently removed.
    pub fn unsubscribe_handler(&mut self, handler: fn(&Event<E>)) -> bool {
        let p_handler: usize;
        unsafe{
            p_handler = *(handler as *const usize);
        }
        match self.handlers.remove(&p_handler){
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
    
    pub fn publish_event_multithreaded(&self, event: &Event<E>){
        let max_concurrency = num_cpus::get();
        let num_threads: usize; 
        
        if max_concurrency == 1 {
            return self.publish_event(event);
        }
        else if max_concurrency <= 64 {
            num_threads = max_concurrency;
        }
        else {
            num_threads = 64;
        }
        
        
        
        let handlers_len = self.handlers.len();
        if handlers_len < num_threads {
            if handlers_len == 1 {
                return self.publish_event(event);
            }
            else {
                for (_, handler) in self.handlers.iter(){
                    let e_clone = event.clone();
                    thread::spawn(move || {
                       handler(e_clone); 
                    });
                }
            }
        }
        else {
            // Attempt to make an optimal number of threads for the number of handlers being pushed to.
            let mut handlers_to_threads: Vec<Vec<fn(&Event<E>)>> = Vec::new();
            for j in 0..num_threads {
                let v: Vec<fn(&Event<E>)> = Vec::new();
                handlers_to_threads.push(v);
            }
            let mut i: usize = 0;
            for (_, handler) in self.handlers.iter(){
                handlers_to_threads[i].push(*handler);
                i = (i + 1) % (num_threads-1);
            }
            
            for handler_vec in handlers_to_threads {
                let e_clone = event.clone();
                thread::spawn(move || {
                    for handler in handler_vec{
                        handler(e_clone);
                    }
                });
            }
        }
    }
}