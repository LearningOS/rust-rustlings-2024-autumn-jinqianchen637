/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/
// I AM DONE

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

// pub struct myStack<T>
// {
// 	//TODO
//     use_q1: bool,
// 	q1:Queue<T>,
// 	q2:Queue<T>
// }
// impl<T> myStack<T> {
//     pub fn new() -> Self {
//         Self {
// 			//TODO
//             use_q1: true,
// 			q1:Queue::<T>::new(),
// 			q2:Queue::<T>::new()
//         }
//     }
//     pub fn push(&mut self, elem: T) {
//         //TODO
//         if self.use_q1 {
//             self.q1.enqueue(elem)
//         } else {
//             self.q2.enqueue(elem)
//         }
//     }
//     pub fn pop(&mut self) -> Result<T, &str> {
//         //TODO
//         if self.use_q1 {
//             match self.q1.size(){
//                 1 => return self.q1.dequeue(),
//                 0 => {
//                     match self.q2.size(){
//                         0 => {return Err("Stack is empty")},
//                         1 => {
//                             return Ok(self.q2.dequeue().unwrap())
//                         },
//                         n => {
//                             self.use_q1 = false;
//                             for _ in 1..=(n-1) {
//                                 self.q1.enqueue(self.q2.dequeue().unwrap());
//                             }
//                             return Ok(self.q2.dequeue().unwrap());
//                         }
//                     }
//                 },
//                 n => {
//                     for _ in 1..=(n-1) {
//                         self.q2.enqueue(self.q1.dequeue().unwrap());
//                     }
//                     self.use_q1 = true;
//                     return Ok(self.q1.dequeue().unwrap());
//                 }
//             }
//         } else {
//             match self.q2.size(){
//                 1 => return self.q2.dequeue(),
//                 0 => {
//                     match self.q1.size(){
//                         0 => {return Err("Stack is empty")},
//                         1 => {
//                             return Ok(self.q1.dequeue().unwrap())
//                         },
//                         n => {
//                             self.use_q1 = true;
//                             for _ in 1..=(n-1) {
//                                 self.q2.enqueue(self.q1.dequeue().unwrap());
//                             }
//                             return Ok(self.q1.dequeue().unwrap());
//                         }
//                     }
//                 },
//                 n => {
//                     for _ in 1..=(n-1) {
//                         self.q1.enqueue(self.q2.dequeue().unwrap());
//                     }
//                     self.use_q1 = true;
//                     return Ok(self.q2.dequeue().unwrap());
//                 }
//             }
//         }
//     }
//     pub fn is_empty(&self) -> bool {
// 		//TODO
//         self.q1.is_empty() && self.q2.is_empty()
//     }
// }


pub struct myStack<T>
{
	//TODO
    cur_queue: u8,
	q1:Queue<T>,
	q2:Queue<T>
}

impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            cur_queue: 0,
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }

    pub fn push(&mut self, elem: T) {
        if self.cur_queue == 0 {
            self.q1.enqueue(elem);
        } else {
            self.q2.enqueue(elem);
        }
    }

    pub fn pop(&mut self) -> Result<T, &str> {
        if self.cur_queue == 0 {
            while true {
                match self.q1.dequeue(){
                    Ok(n) => {
                        if self.q1.is_empty(){
                            self.cur_queue = 1;
                            return Ok(n)
                        } else {
                            {
                                self.q2.enqueue(n)
                            }
                        }
                    },
                    _ => {
                        return Err("Stack is empty")
                    }
                }
            }
            Err("Stack is Empty")
        } else {
            while true {
                match self.q2.dequeue(){
                    Ok(n) => {
                        if self.q2.is_empty(){
                            self.cur_queue = 0;
                            return Ok(n)
                        } else {
                            self.q1.enqueue(n)
                        }
                    },
                    _ => { return Err("Stack is empty")}
                }
            }
            Err("Stack is Empty")
        }
    }

    pub fn is_empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }

}


#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}