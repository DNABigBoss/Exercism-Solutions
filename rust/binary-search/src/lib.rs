pub fn find(array: &[usize], key: usize) -> Option<usize> { 
    if array.is_empty() || array[0] > key {
        return None
    }
    // https://stackoverflow.com/questions/249392/binary-search-in-array
    let mut low = 0;
    let mut high = array.len()-1;

    while low <= high {
        let mid = low + ((high-low)/2);
        if array[mid] > key { high = mid-1 }
        else if array[mid] < key { low = mid+1 }
        else {return Some(mid)}
    }
    None
}
