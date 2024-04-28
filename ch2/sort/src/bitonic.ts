// bitonic sort

// [4, 5, 6, 8]
const main = (arr: number[]) => {
  // 4, 2, 1
  const sort = (arr: number[], up: boolean): number[] => {
    if (arr.length <= 1) {
      return arr;
    }

    const mid = Math.floor(arr.length / 2);
    // 2, 1, 2 (4,5)
    const left = sort(arr.slice(0, mid), true);
    // 2, 1, 2 (6,8)
    const right = sort(arr.slice(mid), false);

    // 1 + 1 (5,4), 1 + 1(7,8), 2 + 2 (4,5,6,8)
    const merged = left.concat(right);

    return subSort(merged, up);
  };

  // ここを並列化するのがバイトニックソートの特徴、この比較は他の処理に依存しないので並列化できる
  // まず半分に割って、その後最小単位（2）まで分割してソート、その後マージしていく
  const subSort = (arr: number[], up: boolean): number[] => {
    if (arr.length <= 1) {
      return arr;
    }
    const mid = Math.floor(arr.length / 2);

    const swappedArr = compareAndSwap(arr, up);

    const left = swappedArr.slice(0, mid);
    const right = swappedArr.slice(mid);

    const sortedLeft = subSort(left, up);
    const sortedRight = subSort(right, up);

    return sortedLeft.concat(sortedRight);
  };

  const compareAndSwap = (arr: number[], up: boolean): number[] => {
    const mid = Math.floor(arr.length / 2);
    for (let i = 0; i < mid; i++) {
      // (5,4), (4,5,6,8)
      if (arr[i] > arr[mid + i] === up) {
        const temp = arr[i];
        arr[i] = arr[mid + i];
        arr[mid + i] = temp;
        // (4,5), (4,5,6,8)
      }
    }
    return arr;
  };

  const res = sort(arr, true);
  console.log(res);
};

main([5, 4]); // [1, 2, 3, 4, 5, 6, 7, 8]
