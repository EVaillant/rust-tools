fn merge_sort_impl_(data: &mut [i64], begin: usize, end: usize) {
    if end - begin > 1 {
        let mid = (end - begin) / 2 + begin;
        merge_sort_impl_(data, begin, mid);
        merge_sort_impl_(data, mid, end);
        for i in begin..mid {
            if data[i] > data[mid] {
                data.swap(mid, i);
                linear_sort_(data, mid, end);
            }
        }
    }
}

fn linear_sort_(data: &mut [i64], begin: usize, end: usize) {
    if end - begin > 1 {
        for i in begin..(end - 1) {
            if data[i] < data[i + 1] {
                break;
            } else {
                data.swap(i, i + 1);
            }
        }
    }
}

pub fn merge_sort(data: &mut [i64]) {
    merge_sort_impl_(data, 0, data.len());
}
