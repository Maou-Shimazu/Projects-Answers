def erastosthenes(num: int) -> set[int]:
    """revised version."""

    evens: set[int] = set()
    for x in range(2, num):
        for y in range(x, num, x):
            if y > x:
                evens.add(y)

    odds: set[int] = {x for x in range(1, num)} - evens
    return sorted(odds)

