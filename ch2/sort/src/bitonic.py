def is_power_of_two(n):
    return (n != 0) and (n & (n - 1) == 0)


def main(arr: list[int]):
    if not is_power_of_two(len(arr)):
        print("Array is not a power of 2")
    if len(arr) <= 1:
        return arr

    print(f"sorting {arr}")

    def sort(arr: list[int], up: bool):
        print(f"sorting {arr} in {up} order")
        if len(arr) <= 1:
            return arr
        else:
            mid = len(arr) // 2
            first = sort(arr[mid:], True)
            second = sort(arr[:mid], False)
            mergedArr = first + second
            return sub_sort(mergedArr, up)

    def sub_sort(arr: list[int], up: bool):
        print(f"sub_sorting {arr} in {up} order")
        if len(arr) <= 1:
            return arr
        else:
            mid = len(arr) // 2
            compare_and_swap(arr, up)
            first = sub_sort(arr[:mid], up)
            second = sub_sort(arr[mid:], up)
            return first + second

    def compare_and_swap(arr: list[int], up: bool):
        mid = len(arr) // 2
        for i in range(mid):
            if (arr[i] > arr[mid + i]) == up:
                arr[i], arr[mid + i] = arr[mid + i], arr[i]

    return sort(arr, True)


if __name__ == "__main__":
    result = main([3, 7, 4, 8, 6, 2, 1, 5, 9, 10, 12, 11, 13, 15, 14, 16])
    print(result)
