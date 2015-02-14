#![crate_id = "event"]
#![crate_type = "rlib"]
#![allow(dead_code)]

use std::rc::Rc;
use std::cmp::Ordering;

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
    handlers: Vec<Rc<Box<Fn(&Event<E>) + 'static>>>,
}

impl<E> EventPublisher<E> {

    /// Event publisher constructor.
    pub fn new() -> EventPublisher<E> {
        EventPublisher{ 
            handlers: Vec::<Rc<Box<Fn(&Event<E>) + 'static>>>::new() 
        }
    }
    /// Subscribes event handler functions to the EventPublisher.
    /// INPUT:  handler_box: Box<Fn(&Event<E>) + 'static>>   handler_box is a box pointer to a function to handle an event of the type E. The function must
    ///     be capable of handling references to the event type set up by the publisher, rather than the raw event itself.
    /// OUTPUT: void
    pub fn subscribe_handler(&mut self, handler_box: Box<Fn(&Event<E>) + 'static>){

        self.handlers.push( Rc::new(handler_box) );
        self.handlers.sort_by(|a,b| (&**a as *const _).cmp(&(&**b as *const _))) 
    }
    
    /// Unsubscribes an event handler from the publisher.
    /// INPUT:  handler_box: Box<Fn(&Event<E>) + 'static>    handler_box is a box pointer to a function to handle an event of the type E.
    /// OUTPUT: bool    output is a bool of whether or not the function was found in the list of subscribed event handlers and subsequently removed.
    pub fn unsubscribe_handler(&mut self, handler_box: Box<Fn(&Event<E>) + 'static>) -> bool {
    
        let len = self.handlers.len();
        
        if len == 0{
            return false;
        }
        
        self.unsub_common_match(handler_box, 0, len / 2, len-1)
    }
    
    /// Internal function to aid unsubscribe_handler and recursive_unsub_search. Match statement that handles the <,>,= comparison of a binary search.
    /// INPUT:  p_handler: *const _     Raw void pointer to the function for the handler.
    ///         l_bound: usize          Lower bound of the binary search indecies.
    ///         mid: usize              Middle of the current binary search boundaries.
    ///         u_bound: usize          Upper bound of the binary search indecies.
    /// OUTPUT: bool                    True/False as to whether or not the event handler function was found and removed from the list.
    fn unsub_common_match(&mut self, handler_box: Box<Fn(&Event<E>) + 'static>, l_bound: usize, mid: usize, u_bound: usize) -> bool {
        let p_handler = &*handler_box as *const _;
        match (p_handler as usize).cmp(&(&**self.handlers[mid] as *const _ as usize)){
            Ordering::Less => {
                if mid == 0{
                    self.recursive_unsub_search(handler_box, l_bound, mid)
                }
                else{
                    self.recursive_unsub_search(handler_box, l_bound, mid-1)
                }
            },
            Ordering::Greater => self.recursive_unsub_search(handler_box, mid, u_bound),
            Ordering:: Equal => {self.handlers.remove(mid); true},
        }
    }
    
    /// Internal function to the unsubscribe_handler process. This is the recursive function that searches and handles boundary conditions.
    /// INPUT:  p_handler: *const _     Raw void pointer to the function for the handler.
    ///         l_bound: usize          Lower bound of the binary search indecies.
    ///         mid: usize              Middle of the current binary search boundaries.
    ///         u_bound: usize          Upper bound of the binary search indecies.
    /// OUTPUT: bool                    True/False as to whether or not the event handler function was found and removed from the list.
    fn recursive_unsub_search(&mut self, handler_box: Box<Fn(&Event<E>) + 'static>, l_bound: usize, u_bound: usize) -> bool {
        let p_handler = &*handler_box as *const _;
        if l_bound == u_bound{
            if p_handler == (&**self.handlers[l_bound] as *const _){
                return true;
            }
            return false;
        }
        
        let mid = l_bound + ((l_bound - u_bound) / 2);
        self.unsub_common_match(handler_box, l_bound, mid, u_bound)
    }
    
    // TODO: Implement this concurrently
    /// Publishes events, pushing the &Event<E> to all handler functions stored by the event publisher.
    /// INPUT: event: &Event<E>     Reference to the Event<E> being pushed to all handling functions.
    pub fn publish_event(&self, event: &Event<E>){
        for handler in self.handlers.iter(){
            handler(event);
        }
    }
}