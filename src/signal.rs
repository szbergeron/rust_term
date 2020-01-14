/*struct Channel<T> {
    queue: std::sync::Arc<lockfree::queue::Queue<T>>,
}

impl Channel<T> {
    fn sender(&self) -> Signal<T> {
        Signal { channel: self.queue.downgrade() }
    }

    fn rcver(&self) -> Recieve<T> {
        Recieve { channel: self.queue.downgrade() }
    }
}

struct Signal<T> {
    channel: std::sync::Weak<Channel<T>>,
}

impl Signal<T> {
    fn send(&self, obj: T) -> bool {
        match self.channel.upgrade() {
            None => false,
            Some(channel) => {
                channel.push(obj);
                true
            }
        }
    }
}

struct Recieve<T> {
    channel: std::sync::Weak<Channel<T>>,
}

impl Recieve<T> {
    fn rcv(&self) -> Option<T> {
        match self.channel.upgrade() {
            None => None,
            Some(channel) => {
                channel.
            }
        }
    }

    fn rcv_iter(&self) -> Option<
}*/
