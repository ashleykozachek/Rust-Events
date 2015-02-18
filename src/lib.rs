#![allow(dead_code)]

use std::rc::Rc;
use std::collections::BTreeMap;

/// Enumerator of the Event type. Whatever type E of Event::Args you implement here is the type E that will be used for the EventPublisher.
pub enum Event<E> {
    Args(E),
    Missing,
}

// To deal with handler functions - F: Rc<Box<Fn(&event<E>)>>
/// EventPublisher. Works similarly to C#'s event publishing pattern. Event handling functions are subscribed to the publisher.
/// Whenever the publisher fires an event it calls all subscribed event handler functions.
/// Use event::EventPublisher::<E>::new() to construct
pub struct EventPublisher<E> {
    //handlers: Vec<Rc<Box<Fn(&Event<E>) + 'static>>>,
    handlers: BTreeMap<usize, Rc<Box<Fn(&Event<E>) + 'static>>>,
}

impl<E> EventPublisher<E> {

    /// Event publisher constructor.
    pub fn new() -> EventPublisher<E> {
        EventPublisher{ 
            //handlers: Vec::<Rc<Box<Fn(&Event<E>) + 'static>>>::new() 
            handlers: BTreeMap::<usize, Rc<Box<Fn(&Event<E>) + 'static>>>::new()
        }
    }
    /// Subscribes event handler functions to the EventPublisher.
    /// INPUT:  handler_box: Box<Fn(&Event<E>) + 'static>>   handler_box is a box pointer to a function to handle an event of the type E. The function must
    ///     be capable of handling references to the event type set up by the publisher, rather than the raw event itself.
    /// OUTPUT: void
    pub fn subscribe_handler(&mut self, handler_box: Box<Fn(&Event<E>) + 'static>){

        //self.handlers.push( Rc::new(handler_box) );
        //self.handlers.sort_by(|a,b| (&**a as *const _).cmp(&(&**b as *const _))) 
        let p_handler = &*handler_box as *const _;
        
        self.handlers.insert(p_handler as usize, Rc::new(handler_box));
    }
    
    /// Unsubscribes an event handler from the publisher.
    /// INPUT:  handler_box: Box<Fn(&Event<E>) + 'static>    handler_box is a box pointer to a function to handle an event of the type E.
    /// OUTPUT: bool    output is a bool of whether or not the function was found in the list of subscribed event handlers and subsequently removed.
    pub fn unsubscribe_handler(&mut self, handler_box: Box<Fn(&Event<E>) + 'static>) -> bool {
        let p_handler = &*handler_box as *const _;
        match self.handlers.remove(&(p_handler as usize)){
            Some(_) => true,
            None => false,
        }
    }
        
    // TODO: Implement this concurrently
    /// Publishes events, pushing the &Event<E> to all handler functions stored by the event publisher.
    /// INPUT: event: &Event<E>     Reference to the Event<E> being pushed to all handling functions.
    pub fn publish_event(&self, event: &Event<E>){
        for (_, handler) in self.handlers.iter(){
            handler(event);
        }
    }
}