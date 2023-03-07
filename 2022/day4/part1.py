

def line_to_ranges(line: str) -> tuple[tuple[int, int], tuple[int, int]]:
    a, b = line.split(",")
    return tuple(map(lambda d: int(d), a.split('-'))), tuple(map(lambda d: int(d), b.split('-')))


def is_contained(ranges) -> bool:
    a, b = ranges
    return (b[0] >= a[0] and b[1] <= a[1]) or (a[0] >= b[0] and a[1] <= b[1])


def main():
    with open("input.txt") as input_file:
        count = sum(map(is_contained, map(line_to_ranges, input_file)))
    print(f"the count is {count}")


# print(is_contained(line_to_ranges("2-3,3-7")))

if __name__ == "__main__":
    main()