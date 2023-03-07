

def line_to_ranges(line: str) -> tuple[tuple[int, int], tuple[int, int]]:
    a, b = line.split(",")
    return tuple(map(lambda d: int(d), a.split('-'))), tuple(map(lambda d: int(d), b.split('-')))


def is_overlap(ranges) -> bool:
    a, b = ranges
    return any((b[0] <= a[0] <= b[1],
               b[0] <= a[1] <= b[1],
                a[0] <= b[0] <= a[1],
                a[0] <= b[1] <= a[1]))


def main():
    with open("input.txt") as input_file:
        count = sum(map(is_overlap, map(line_to_ranges, input_file)))
    print(f"the count is {count}")


# print(is_contained(line_to_ranges("2-3,3-7")))

def overlap_test():
    assert (is_overlap(((1, 3), (3, 4))) == True)
    assert (is_overlap(((1, 3), (4, 4))) == False)
    assert (is_overlap(((1, 1), (2, 2))) == False)
    assert (is_overlap(((1, 1), (0, 4))) == True)
    assert (is_overlap(((1, 3), (2, 4))) == True)
    assert (is_overlap(((3, 4), (1, 3))) == True)
    assert (is_overlap(( (4, 4), (1,3))) == False)
    assert (is_overlap(((2, 2),(1,1))) == False)
    assert (is_overlap(((0, 4),(1,1))) == True)
    assert (is_overlap(( (2, 4),(1,3))) == True)


if __name__ == "__main__":
    main()
    overlap_test()
