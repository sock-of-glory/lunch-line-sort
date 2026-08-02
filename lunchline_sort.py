nums = [42, 7, 91, 13, 56, 2, 88, 34, 19, 73, 5, 60, 27, 100, 11]

def sort_list(nums):
    i = 1
    while i < len(nums):
        if nums[i - 1] > nums[i]:
            num = nums.pop(i - 1)
            nums.append(num)
            i = 1
        else:
            i += 1
    return nums
        
print(sort_list(nums))