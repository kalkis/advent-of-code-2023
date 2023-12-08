from typing import List
from math import lcm
from functools import reduce


def main(filename: str):
    directions = { 'L': 0, 'R': 1}
    network, instructions, starting_nodes = build_network(filename)
    part_one(network, directions, instructions, "AAA", "ZZZ")
    part_two(network, directions, instructions, starting_nodes, 'Z')
    

def build_network(filename: str) -> (dict, List[str], List[str]):
    network = {}
    starting_nodes = []
    with open(filename) as f:
        instructions = f.readline().strip()
        f.readline()

        for line in f:
            line = line.strip()
            node = line[0:3]
            if node.endswith('A'):
                starting_nodes.append(node)
            left = line[7:10]
            right = line[12:15]
            network[node] = (left, right) 
    return (network, instructions, starting_nodes)


def part_one(network: dict, directions: dict, instructions: List[str], starting_node: str, exit_node: str):
    steps = find_exit(network, directions, instructions, starting_node, exit_node)
    print(f"Steps from {starting_node} -> {exit_node}: {steps}")


def find_exit(network: dict, directions: dict, instructions: List[str], starting_node: str, exit_pattern: str) -> int:
    steps = 0
    current_node = starting_node
    current_instruction = directions[instructions[0]]
    while not current_node.endswith(exit_pattern):
        current_node = network[current_node][current_instruction]
        steps += 1
        current_instruction = directions[instructions[steps % len(instructions)]]
    return steps


def reduce_cycles(network: dict, directions: dict, instructions: List[str], starting_nodes: List[str], exit_pattern: str) -> int:
    cycles = []
    for node in starting_nodes:
        steps = find_exit(network, directions, instructions, node, exit_pattern)
        cycles.append(steps)
    # lowest common multiple of all cycles is first number of steps until all nodes end with 'Z'
    return reduce(lcm, cycles)


def part_two(network: dict, directions: dict, instructions: List[str], starting_nodes: List[str], exit_pattern: str):
    steps = reduce_cycles(network, directions, instructions, starting_nodes, exit_pattern)
    print(f"Steps from {starting_nodes} -> exit nodes: {steps}")
 

if __name__ == '__main__':
    main("input.txt")