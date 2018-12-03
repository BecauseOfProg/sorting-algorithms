/// Sorts a list using bubble algorithm.
pub fn bubble_sort(t: Vec<usize>) -> Vec<usize> {
    let mut new_t = t;
    let mut sorted: bool = false;
    while sorted == false {
        sorted = true;
        for i in 0..new_t.len() - 1 {
            if new_t[i + 1] < new_t[i] {
                let temp = new_t[i + 1];
                new_t[i + 1] = new_t[i];
                new_t[i] = temp;
                sorted = false;
            }
        }
    }
    new_t
}
