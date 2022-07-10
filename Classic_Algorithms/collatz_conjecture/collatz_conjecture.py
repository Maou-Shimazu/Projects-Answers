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


def visualize(seq: list[int | float] | list[list[int | float]]) -> None:
    """Create a graph of the sequence, requires matplotlib.

    Arguments:
    seq -- maybe the sequence or list of sequences.
    """

    import matplotlib.pyplot as plt

    fig, ax = plt.subplots(figsize=(10,3))
    plt.title("Collatz Conjecture of the Given Sequence.")

    if isinstance(seq[0], list):
        for sequence in seq:
            ax.plot(sequence)
    else:
        ax.plot(seq)

    fig = plt.gcf()
    plt.show()
    plt.draw()
    fig.savefig("collatz.png", dpi=300, bbox_inches="tight")
