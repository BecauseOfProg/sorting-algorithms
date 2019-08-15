// Sort a list using the Bubble sort algorithm.
func BubbleSort(t []float64) []float64 {
	sorted := false
	for sorted == false {
		sorted = true
		for i := 0; i < len(t)-1; i++ {
			if t[i] > t[i+1] {
				t[i+1], t[i] = t[i], t[i+1]
				sorted = false
			}
		}
	}
	return t
}