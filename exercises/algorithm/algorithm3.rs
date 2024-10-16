/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
// I AM DONE

fn sort<T>(array: &mut [T])
    where T: std::cmp::PartialOrd + Clone
    {
	//TODO
    // fast sort
    match array.len() {
        0..=1 => return,
        2 => {if array[0] > array[1]{
            (array[0], array[1])  = (array[1].clone(), array[0].clone());
        }},
        _ => {
            let arr_len = array.len();
            let mut i_left = 0_usize;
            let mut i_right = array.len() -1;
            let val_cmp = &array[i_left].clone();
            let mut i_focus = 0_usize;
            for _ in 0..(array.len()) {
                if array[i_focus] <= *val_cmp {
                    i_focus += 1;
                    i_left += 1;
                } else {
                    (array[i_focus], array[i_right])  = (array[i_right].clone(), array[i_focus].clone());
                    i_right -= 1;
                }
            }
            (array[i_left-1], array[0])  = (array[0].clone(), array[i_left-1].clone());
            sort(&mut array[0..(i_left - 1)]);
            sort(&mut array[i_left..arr_len]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}