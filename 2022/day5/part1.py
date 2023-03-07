stacks = [
    "BLDTWCFM",
    "NBL",
    "JCHTLV",
    "SPJW",
    "ZSCFTLR",
    "WDGBHNZ",
    "FMSPVGCN",
    "WQRJFVCZ",
    "RPMLH",
]
stacks = [list(stack) for stack in stacks]


def parse_instruction(s: str) -> tuple[int, int, int]:
    words = s.split()
    return int(words[1]), int(words[3]) - 1, int(words[5]) - 1


assert parse_instruction("move 5 from 3 to 6") == (5, 2, 5)


def execute_instruction_p1(stacks, amount, _from, to):
    for _ in range(amount):
        stacks[to].append(stacks[_from].pop())


def execute_instruction_p2(stacks, amount, _from, to):
    stacks[to].extend(stacks[_from][-amount:])
    stacks[_from] = stacks[_from][:-amount]


def print_tops(stacks):
    for stack in stacks:
        print(stack[-1], end=", ")


def main():
    with open("input.txt") as f:
        for line in f:
            # print(line)
            execute_instruction_p2(stacks, *parse_instruction(line))
    print_tops(stacks)


if __name__ == "__main__":
    main()
