use rand::prelude::*;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
struct CmpVaule {
    value: i32,
}

impl PartialEq for CmpVaule {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl PartialOrd for CmpVaule {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    // generate random numbers
    let mut nums: Vec<i32> = (1..100).collect();
    nums.shuffle(&mut rng);

    // make vec to array
    let mut num_arr: [i32; 99] = [0; 99];
    let mut i = 0;
    for n in nums {
        num_arr[i] = n;
        i += 1;
    }

    println!("Before sort: {:?} \n", num_arr);

    // sort happen in here
    bubble_sort(&mut num_arr);

    println!("After sort: {:?}", num_arr);
}

/**
 * sort array
 */
fn bubble_sort<T>(arr: &mut [T])
where
    T: PartialOrd + Copy,
{
    let mut swapped = true;

    while swapped {
        let mut i = 0;
        swapped = false;
        while i < arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                let tmp = arr[i];
                arr[i] = arr[i + 1];
                arr[i + 1] = tmp;
                swapped = true;
            }
            i += 1;
        }
    }
}
