def erastosthenes(num: int) -> set[int]:
    """revised version."""

    evens: set[int] = set()
    for x in range(2, num):
        if x == 2:
            for y in range(2, num, 2):
                if y > x:
                    evens.add(y)
        else:
            for y in range(x, num, x):
                if y > x:
                    evens.add(y)

    odds: set[int] = {x for x in range(1, num)} - evens
    return sorted(odds)

