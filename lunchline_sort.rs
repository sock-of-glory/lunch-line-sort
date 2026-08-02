fn main() {
    let nums: &mut Vec<i32> = &mut Vec::new();
    *nums = vec![42, 7, 91, 13, 56, 2, 88, 34, 19, 73, 5, 60, 27, 100, 11];

    println!("{:?}", sort_list(nums));
}

fn sort_list(nums: &mut Vec<i32>) -> &mut Vec<i32> {
    let mut i: usize = 1;
    while i < nums.len() {
        if nums[i-1] > nums[i] {
            let num: i32 = nums.remove(i - 1);
            nums.push(num);
            i = 1;
        } else {
            i += 1;
        }
    }
    return nums;
}