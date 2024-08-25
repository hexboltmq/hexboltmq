#[derive(Debug)]
pub struct Message {
    pub id: String,
    pub body: Vec<u8>,
}

pub struct Queue {
    pub name: String,
    messages: Vec<Message>,
    message_count: u32,
}

impl Queue {
    pub fn new(name: &str) -> Self {
        Queue {
            name: name.to_string(),
            messages: Vec::new(),
            message_count: 0,
        }
    }

    pub fn get_message_count(&self) -> u32 {
        self.message_count
    }

    pub fn enqueue(&mut self, message: Message) {
        println!("Enqueuing message:  id.<{:?}>  message.<{:?}>", message.id, message.body);
        self.messages.push(message);
        self.message_count += 1;
    }

    pub fn dequeue(&mut self) -> Option<Message> {
        let message = self.messages.pop();
        self.message_count -= 1;
        return message;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue_dequeue() {
        let mut queue = Queue::new("test_queue");
        let message = Message {
            id: "1".to_string(),
            body: b"Hello, Hexbolt!".to_vec(),
        };

        queue.enqueue(message);
        assert_eq!(queue.messages.len(), 1);

        let dequeued_message = queue.dequeue().unwrap();
        assert_eq!(dequeued_message.id, "1");
        assert_eq!(dequeued_message.body, b"Hello, Hexbolt!".to_vec());
    }
}
