quickSort<T: Ord>: (mut items: Vec<T>) = {
  partition: (mut items: Vec<T>, low, hight: i32) -> i32 = {
    pivot = items[high];
    i = low - 1;
    for j=low; j<= high -1; j++ {
      if items[j] < pivot {
        i+=1;
        items[i], items[j] = items[j], items[i];
      }
    }
    i + 1
  }

  sort: (mut items: Vec<T>, low: i32, high: i32) = {
    if low < high {
      pi = partition(items, low, high);
      quickSort(items, low, pi - 1);
      quickSort(items, pi + 1, high);
    }
  }

  sort(items, 0, items.length() - 1);
}