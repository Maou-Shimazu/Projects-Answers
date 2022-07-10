def validate(card_num: str) -> bool:
    """Using this method: https://www.sapling.com/7966257/checksum-credit-card"""
    try:
        print(card_num)
        seq: list[int] = [
                int(x)*2 if i % 2 ==0 else int(x)
                for i, x in enumerate(card_num.replace(" ", ""))
            ]
        total: int = 0
        for num in seq[:-1]:
            if len(str(num)) == 2:
                total += int(str(num)[0]) + int(str(num)[1])
            else:
                total += num

        total += seq[-1]
        if total % 10 == 0:
            return True

    except (ValueError, IndexError):
        print("Error occured.")

    return False
