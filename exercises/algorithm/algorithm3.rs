/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
fn quick<T: PartialOrd + std::clone::Clone >(array: &mut [T], l: usize, r: usize) {
    if l >= r { return; }

    let mut i = l;  let mut j = r;
    let t = array[l].clone();

    while i < j {
        while array[j] >= array[l] && i < j{
            j -= 1;
        }
        while array[i] <= array[l] && i < j{
            i += 1;
        }
        array.swap(j, i);
    }
    array.swap(i, l);

    quick(array, l, i);
    quick(array, i + 1, r);
}

fn sort<T: PartialOrd + std::clone::Clone>(array: &mut [T]){
	//TODO
    quick(array, 0 as usize , array.len()-1 as usize);
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