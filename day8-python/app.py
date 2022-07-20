# Array of tuples (signal_patterns, digit_patterns)
input_values = []

file = open("day8-python/input.txt", "r")
for line in file:
    if (line != ""):
        [raw_signal_patterns, raw_digits] = line.strip().split(" | ")
        signal_patterns = raw_signal_patterns.split(" ")
        digit_patterns = raw_digits.split(" ")
        input_values.append((signal_patterns, digit_patterns))


LENGTH_TO_DIGIT = {
    2: 1,
    3: 7,
    4: 4,
    7: 8
}

LETTERS_TO_DIGIT = {
    "abcefg": 0,
    "cf": 1,
    "acdeg": 2,
    "acdfg": 3,
    "bcdf": 4,
    "abdfg": 5,
    "abdefg": 6,
    "acf": 7,
    "abcdefg": 8,
    "abcdfg": 9
}

DIGIT_TO_LETTERS = {
    0: "abcefg",
    1: "cf",
    2: "acdeg",
    3: "acdfg",
    4: "bcdf",
    5: "abdfg",
    6: "abdefg",
    7: "acf",
    8: "abcdefg",
    9: "abcdfg"
}

# { digit: number of appearances }
digit_tracker = {
    1: 0, 4: 0, 7: 0, 8: 0
}

for (signal_patterns, output_patterns) in input_values:

    possible_mappings = { "a": "abcdefg", "b": "abcdefg", "c": "abcdefg", "d": "abcdefg",
        "e": "abcdefg", "f": "abcdefg", "g": "abcdefg" }

    for signal_pattern in signal_patterns:
        
        # See if we recognize this digit by the length of the signal pattern
        found_digit = LENGTH_TO_DIGIT.get(len(signal_pattern))
        if found_digit != None:
            correct_letters = DIGIT_TO_LETTERS[found_digit]

            # Loop through jumbled letters and update the possible mappings
            for jumbled_letter in signal_pattern:
                current_letter_mapping = possible_mappings[jumbled_letter]
                for letter in current_letter_mapping:
                    if correct_letters.find(letter) == -1:
                        possible_mappings[jumbled_letter] = current_letter_mapping.replace(letter, "")
    
    for output_pattern in output_patterns:
        
        # See if we recognize this digit by the length of the output pattern
        found_digit = LENGTH_TO_DIGIT.get(len(output_pattern))
        if found_digit != None:
            correct_letters = DIGIT_TO_LETTERS[found_digit]

            # Loop through jumbled letters and update the possible mappings
            for jumbled_letter in output_pattern:
                current_letter_mapping = possible_mappings[jumbled_letter]
                for letter in current_letter_mapping:
                    if correct_letters.find(letter) == -1:
                        possible_mappings[jumbled_letter] = current_letter_mapping.replace(letter, "")
    
    print(possible_mappings)
    break

#     for digit_pattern in digit_patterns:
#         found_digit = LENGTH_TO_DIGIT.get(len(digit_pattern))
#         if found_digit != None:
#             digit_tracker[found_digit] += 1

# print(digit_tracker)

# sum = 0
# for num in digit_tracker.values():
#     sum += num

# print(sum)