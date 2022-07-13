//@ts-check
const { createReadStream } = require('fs')
const readline = require('readline')

const main = async () => {
    const { bingoBoards, drawnNums } = await readInputsFromFile();

    /** @type {{ rounds: number; score: number } | null} */
    let bestBoard = null;
    /** @type {{ rounds: number; score: number } | null} */
    let worstBoard = null;

    for (const board of bingoBoards) {
        const boardResults = playBoard(board, drawnNums);
        if (boardResults) {
            if (!bestBoard) bestBoard = boardResults;
            else if (boardResults.rounds < bestBoard.rounds) bestBoard = boardResults;

            if (!worstBoard) worstBoard = boardResults;
            else if (boardResults.rounds > worstBoard.rounds) worstBoard = boardResults;
        }
    }

    console.log(`The best board will win in ${bestBoard?.rounds} rounds and have a final score of ${bestBoard?.score}`)
    console.log(`The worst board will win in ${worstBoard?.rounds} rounds and have a final score of ${worstBoard?.score}`)
}

main();

/**
 * Play a game with the given board and drawn numbers, and - if they have a winning row/column -
 * the # of rounds to win as well as the final score
 * 
 * @param {Array<{ num: number, marked: boolean }>} board the Bingo board
 * @param {number[]} drawnNums the drawn numbers
 */
 function playBoard(board, drawnNums) {

    // For each round
    for (let round = 0; round < drawnNums.length; round++) {
        let drawnNum = drawnNums[round];

        // Try to find the drawn number on the board, and mark it if found
        const foundCell = board.find(cell => cell.num === drawnNum);
        if (foundCell) foundCell.marked = true;

        // See if we've won. If so, return # of rounds and score
        if (round >= 4 && foundCell && isWinningBoard(board)) {
            const sumOfUnmarkedCells = board
                .filter(c => !c.marked)
                .reduce((sum, current) => sum += current.num, 0);
            const score = sumOfUnmarkedCells * drawnNum;

            return { rounds: round, score };
        }
    }
    // If we get here, we didn't win anything 😢
    return null;
}

/**
 * Check if the board has a completely marked row or column
 * @param {Array<{ num: number, marked: boolean }>} board the Bingo board 
 */
function isWinningBoard(board) {
    
    // Check rows (consecutive groups of 5 cells)
    for (let i = 0; i < board.length; i += 5) {
        const isWinningRow = board.slice(i, i + 5).every(cell => cell.marked === true);
        if (isWinningRow) return true;
    }

    // Check columns (every 5 cells => e.g. indices [0, 5, 10, 15, 20], [1, 6, 11, 16, 21])
    for (let column = 0; column < 5; column++) {
        const columnCells = board.filter((_, idx) => (idx - column) % 5 === 0);
        const isWinningColumn = columnCells.every(cell => cell.marked === true);
        if (isWinningColumn) return true;
    }

    return false;
}

/**
 * Read inputs from file. Returns the drawn numbers, and all bingo boards.
 * (This was not fun in JS. Much more fun to read files in Python and Rust)
 * 
 * @returns {Promise<{ 
 *     bingoBoards: Array<{ num: number, marked: boolean }>[], 
 *     drawnNums: number[]  
 * }>}
 */
function readInputsFromFile() { return new Promise((resolve, reject) => {
    
    /* @type {Array<{ num: number, marked: boolean }>[]} */
    const bingoBoards = [];
    /** @type {number[]} */
    const drawnNums = [];

    const readStream = createReadStream('input.txt')
    const lineReader = readline.createInterface({
        input: readStream, crlfDelay: Infinity
    })
    let lineNum = 1;
    let boardIndex = 0;
    lineReader.on("line", line => {

        // process first line as drawn numbers
        if (lineNum === 1) drawnNums.push(...line.split(",").map(s => parseInt(s)));

        // process end of a board
        else if (line === "" && bingoBoards[boardIndex]?.length === 25) {
            boardIndex++;
        }
    
        // process board
        else if (line) {
            // initialize new board if needed
            if (!bingoBoards[boardIndex]) bingoBoards[boardIndex] = [];
    
            // read board numbers
            const boardNums = line.split(/\s+/) // split numbers by whitespace
                .filter(s => s != "")
                .map(s => ({ num: parseInt(s), marked: false }));
            if (boardNums.length !== 5) throw `unexpected length of board numbers at line ${lineNum}`
    
            // add numbers to current board
            bingoBoards[boardIndex].push(...boardNums);
        }
    
        lineNum++;
    })
    .on('close', () => resolve({ bingoBoards, drawnNums }))
    .on('error', (err) => reject(err))
})}
