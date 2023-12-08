from typing import List    # allows for type hint annotations

def main(filename: str):
    part_one(filename)
    part_two(filename)


def part_one(filename: str):
    total_points = calculate_points(filename)
    print("Total points of scratchcards: " + str(total_points)) 


def part_two(filename: str):
    total_cards = calculate_cards(filename)
    print("Total number of scratchcards won: " + str(total_cards))  


def calculate_points(filename: str) -> int:
    total_points = 0
    with open(filename) as f:
        for line in f:
            numbers = line.strip().split(':')[1]
            matches = parse_card_matches(numbers)
            if matches > 0:
                total_points += 2 ** (matches - 1)
    return total_points


def parse_card(card: str):
    card_num, numbers = card.split(':')
    card_num = parse_card_number(card_num)
    matches = parse_card_matches(numbers)
    wins = [card_num + i for i in range(1, matches + 1)]
    return (card_num, wins)


def parse_card_number(card_num: str):
    return int(card_num.split(' ')[-1])


def parse_card_matches(numbers: List[str]) -> int:
    winning_numbers, my_numbers = [[int(x) for x in y.split(' ') if x.isdigit()] for y in numbers.split('|')]
    return sum([x in winning_numbers for x in my_numbers])


def calculate_cards(filename: str) -> int:
    # make dictionary of all cards and number of winning matches
    card_totals = {}
    with open(filename) as f:
        for line in f:
            card_num, wins = parse_card(line.strip())
            num_copies = card_totals.setdefault(card_num, 1)
            for n in wins:
                card_totals.update({n : card_totals.get(n, 1) + num_copies})
    return sum(card_totals.values())


if __name__ == '__main__':
    main("input.txt")