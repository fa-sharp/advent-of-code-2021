# Get array of depth measurements from input file

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
    4: 4,
    3: 7,
    7: 8
}

# { digit: number of appearances }
digit_tracker = {
    1: 0, 4: 0, 7: 0, 8: 0
}

for (_, digit_patterns) in input_values:
    for digit_pattern in digit_patterns:
        found_digit = LENGTH_TO_DIGIT.get(len(digit_pattern))
        if found_digit != None:
            digit_tracker[found_digit] += 1

print(digit_tracker)

sum = 0
for num in digit_tracker.values():
    sum += num

print(sum)