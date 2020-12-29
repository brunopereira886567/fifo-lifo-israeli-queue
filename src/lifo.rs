pub struct Lifo<T> {
    lifo: Vec<T>,
  }
  
  impl<T> Lifo<T> {
    pub fn new() -> Self {
        Lifo { lifo: Vec::new() }
    }
  
    pub fn length(&self) -> usize {
      self.lifo.len()
    }
  
    pub fn pop(&mut self) -> Option<T> {
      self.lifo.pop()
    }
  
    pub fn push(&mut self, item: T) {
      self.lifo.push(item)
    }
  
    pub fn is_empty(&self) -> bool {
      self.lifo.is_empty()
    }
  
    pub fn peek(&self) -> Option<&T> {
      self.lifo.last()
    }
  }

  fn main(){
    let mut lifo: Lifo<isize> = Lifo::new();
    lifo.push(1);
    lifo.push(2);
    lifo.push(0);
    lifo.push(4);
    
    let item = lifo.pop();
    assert_eq!(item.unwrap(), 4);
    assert_eq!(lifo.is_empty(), false);

    let item = lifo.pop();
    assert_eq!(item.unwrap(), 0);
    assert_eq!(lifo.is_empty(), false);

    let item = lifo.pop();
    assert_eq!(item.unwrap(), 2);
    assert_eq!(lifo.is_empty(), false);

    let item = lifo.pop();
    assert_eq!(item.unwrap(), 1);
    assert_eq!(lifo.is_empty(), true);

}