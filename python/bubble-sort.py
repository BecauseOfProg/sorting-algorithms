def bubble_sort(T):
    """Sorts a list using bubble sort.
    :param a: List to sort
    :type a: list

    :returns: Sorted list
    :rtype: list"""
    ordered = False
    while not ordered:
        ordered = True
        for i in range(0, len(T)-1):
            if T[i+1] < T[i]:
                temp = T[i+1]
                T[i+1] = T[i]
                T[i] = temp
                ordered = False
    return T
