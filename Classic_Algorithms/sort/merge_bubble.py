from typing import NewType


Arr = NewType("Arr", list[int | float])


def bubble_sort(arr: Arr) -> Arr:
    """bubble buddy."""

    for _ in arr:
        for i, x in enumerate(arr):
            try:
                if x > arr[i+1]:
                    val = arr[i+1]
                    arr[i+1] = x
                    arr[i] = val
            except IndexError:
                pass

    return arr

