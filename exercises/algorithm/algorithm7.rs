/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/

// I AM DONE

use std::cell::RefCell;

#[derive(Debug)]
struct Stack<T> {
	size: usize,
	data: Vec<T>,
}
impl<T> Stack<T> {
	fn new() -> Self {
		Self {
			size: 0,
			data: Vec::new(),
		}
	}
	fn is_empty(&self) -> bool {
		0 == self.size
	}
	fn len(&self) -> usize {
		self.size
	}
	fn clear(&mut self) {
		self.size = 0;
		self.data.clear();
	}
	fn push(&mut self, val: T) {
		self.data.push(val);
		self.size += 1;
	}
	fn pop(&mut self) -> Option<T> {
		let rlt = self.data.pop();
		match rlt {
			None => None,
			_ => {
				self.size -= 1;
				rlt
			}
		}
	}
	fn peek(&self) -> Option<&T> {
		if 0 == self.size {
			return None;
		}
		self.data.get(self.size - 1)
	}
	fn peek_mut(&mut self) -> Option<&mut T> {
		if 0 == self.size {
			return None;
		}
		self.data.get_mut(self.size - 1)
	}
	fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}
	fn iter(&self) -> Iter<T> {
		let mut iterator = Iter { 
			stack: Vec::new() 
		};
		for item in self.data.iter() {
			iterator.stack.push(item);
		}
		iterator
	}
	fn iter_mut(&mut self) -> IterMut<T> {
		let mut iterator = IterMut { 
			stack: Vec::new() 
		};
		for item in self.data.iter_mut() {
			iterator.stack.push(item);
		}
		iterator
	}
}
struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		if !self.0.is_empty() {
			self.0.size -= 1;self.0.data.pop()
		} 
		else {
			None
		}
	}
}
struct Iter<'a, T: 'a> {
	stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}
struct IterMut<'a, T: 'a> {
	stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}

fn pare(c: char) -> Option<char> {
	match c {
		'(' => Some(')'),
		'[' => Some(']'),
		'{' => Some('}'),
		')' => Some('('),
		']' => Some('['),
		'}' => Some('{'),
		c => None
	}
}

fn bracket_match(bracket: &str) -> bool {
	let mut rc_stack = RefCell::new(Stack::new());
	bracket.chars().fold(
		true, |x, c|{
			if !x {
				false
			}else{
				match c {
					c if c == '(' || c == '[' || c == '{'  => {
						rc_stack.borrow_mut().push(c);
						x
					},
					c if c == ')' || c == ']' || c == '}' => {
						match rc_stack.borrow_mut().pop(){
							None => {false},
							Some(cp) => {
								match pare(cp){
									None => { false },
									Some(c_p) => { c_p == c }
								}
							}
						}
					},
					_c => { x },
				}
			}
		}) 
		&& rc_stack.borrow_mut().is_empty()
}

// fn bracket_match_2(bracket: &str) -> bool
// {
// 	//TODO, how to do it with fold?
// 	let mut s = Stack::new();
// 	let sref = &mut s;
// 	let mut rlt = true;
// 	for c in bracket.chars() {
// 		println!("# c {}",c);
// 		if !rlt { return false };
// 		rlt = {
// 				match c {
// 					c if c == '(' || c == '[' || c == '{' => {
// 						sref.push(c);
// 						println!("# push {}",c);
// 						rlt
// 					},
// 					c if c == ')' || c == ']' || c == '}' => {
// 						match sref.pop() {
// 							None => { false }
// 							Some(s) => {
// 								println!("# pop {}", s);
// 								match pare(c){
// 									None => false,
// 									Some(c_p) => {
// 										if c_p == s {
// 											true
// 										} else {
// 											false
// 										}
// 									}
// 								}
// 							}
// 						}
// 					},
// 					_ => { 
// 						rlt
// 					}
// 				}
// 			}
// 	}
// 	rlt && sref.is_empty()
// }

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn bracket_matching_1(){
		let s = "(2+3){func}[abc]";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_2(){
		let s = "(2+3)*(3-1";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_3(){
		let s = "{{([])}}";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_4(){
		let s = "{{(}[)]}";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_5(){
		let s = "[[[]]]]]]]]]";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_6(){
		let s = "";
		assert_eq!(bracket_match(s),true);
	}
}