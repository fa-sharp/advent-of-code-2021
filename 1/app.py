#### Part 1 - Find out how many times the depth increases https://adventofcode.com/2021/day/1

# Get array of depth measurements from input file
file = open("input.txt", "r")
depthMeasurements = []
for line in file:
    if (line != ""):
        depthMeasurements.append(int(line))

# Calculate how many times the depth increased, by iterating over the array and comapring prev to current
numIncreased = 0
prevDepth = None
for currentDepth in depthMeasurements:
    if (prevDepth != None and currentDepth > prevDepth):
        numIncreased += 1
    prevDepth = currentDepth

print(f"The depth increased: {numIncreased} times")


##### Part 2 - Using a three-measurement sliding window, how many times did the depth increase?

# keep track of the window using a starting and ending cursor of indices in the array
startCursor = 0
endCursor = 2

# keep track of previous sum, similar to before
prevSum = None
numWindowIncreased = 0

while endCursor < len(depthMeasurements):

    currentSum = sum(depthMeasurements[startCursor:endCursor + 1])

    if (prevSum != None and currentSum > prevSum):
        numWindowIncreased += 1
    
    prevSum = currentSum
    startCursor += 1
    endCursor += 1

print(f"Using a 3-measurement window, the depth increased: {numWindowIncreased} times")
