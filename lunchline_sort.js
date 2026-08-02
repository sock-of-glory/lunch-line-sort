let nums = [42, 7, 91, 13, 56, 2, 88, 34, 19, 73, 5, 60, 27, 100, 1];

function sort_list(nums) {
    let i = 1;
    while (i < nums.length) {
        if (nums[i - 1] > nums[i]) {
            let num = nums.splice(i, 1);
            nums.push(num);
            i = 1;
        } else {
            i++;
        }
    }
}