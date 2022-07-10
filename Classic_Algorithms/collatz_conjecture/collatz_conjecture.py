def collatz(num: int) -> list[int | float]:
    """Given the number, find the sequence."""

    n_arr: list[int | float] = []

    while True:
        n_arr.append(num)

        if num % 2 == 0:
            num = num/2
        else:
            num = 3*num + 1

        if num == 1:
            break

    return n_arr

