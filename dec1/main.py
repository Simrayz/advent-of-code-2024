
import sys

def get_location_id_lists(path: str) -> tuple[list[int], list[int]]: 
    left_numbers = []
    right_numbers = []

    with open(path) as f:
        for line in f:
            numbers = [int(x) for x in line.split()]
            left_numbers.append(numbers[0])
            right_numbers.append(numbers[1])

    sorted_left = sorted(left_numbers)
    sorted_right = sorted(right_numbers)

    return sorted_left, sorted_right

def get_distance_score(sorted_left: list[int], sorted_right: list[int]) -> list[int]:
    distance_score = 0

    for pair in zip(sorted_left, sorted_right):
        distance_score += abs(pair[0] - pair[1])

    return distance_score

def get_similarity_score(sorted_left: list[int], sorted_right: list[int]) -> int:
    frequency_dict = create_frequency_dict(sorted_right)
    similarity_score = 0
    for number in sorted_left:
        if number in frequency_dict:
            similarity_score += number * frequency_dict[number]
            
    return similarity_score
    
def create_frequency_dict(numbers: list[int]) -> dict[int, int]:
    frequency_dict = {}
    for number in numbers:
        if number in frequency_dict:
            frequency_dict[number] += 1
        else:
            frequency_dict[number] = 1
    return frequency_dict

def main():
    sorted_left, sorted_right = get_location_id_lists(sys.argv[1])
    distance_score = get_distance_score(sorted_left, sorted_right)
    similarity_score = get_similarity_score(sorted_left, sorted_right)

    print("Distance   score: ", distance_score)
    print("Similarity score: ", similarity_score)

if __name__ == "__main__":
    main()