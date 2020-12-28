 
pub struct Queue<T> {
    queue: Vec<T>,
  }
  
  impl<T> Queue<T> {
    pub fn new() -> Self {
      Queue { queue: Vec::new() }
    }
  
    pub fn length(&self) -> usize {
      self.queue.len()
    }
  
    pub fn enqueue(&mut self, item: T) {
      self.queue.push(item)
    }
  
    pub fn dequeue(&mut self) -> T {
      self.queue.remove(0)
    }
    pub fn is_empty(&self) -> bool {
      self.queue.is_empty()
    }
  
    pub fn peek(&self) -> Option<&T> {
      self.queue.first()
    }
  }
fn main(){
    let mut queue: Queue<isize> = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(0);
    queue.enqueue(4);
    let item = queue.dequeue();
    assert_eq!(item, 1);
    assert_eq!(queue.is_empty(), false);

    let item = queue.dequeue();
    assert_eq!(item, 2);
    assert_eq!(queue.is_empty(), false);

    let item = queue.dequeue();
    assert_eq!(item, 0);
    assert_eq!(queue.is_empty(), false);

    let item = queue.dequeue();
    assert_eq!(item, 4);
    assert_eq!(queue.is_empty(), true);

}