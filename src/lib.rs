enum Event<E> {
    Args(E),
    Missing,
}

struct EventHandler<F)> {
    e: &Event,
    handlers: &mut Vec<F>,
}

#[test]
fn it_works() {
}
