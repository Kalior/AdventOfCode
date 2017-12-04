package day3

import kotlin.system.measureTimeMillis

fun main(args : Array<String>) {
    val mil = measureTimeMillis {
        val input = parse()
        solve1(input)
        solve2(input)
    }
    println(mil)
}

fun parse() : String {
    val parser = util.Parser("3")
    val lines = parser.getLines()
    return lines[0]
}

fun solve2(input: String) : Unit {
    val inputNbr = input.toInt()

    var grid = Array(10, {intArrayOf(0,0,0,0,0,0,0,0,0,0)})

    grid[4][4] = 1
    var x = 5
    var y = 4
    var level = 1
    var addedInLevel = 0
    var next = 0
    while (next < inputNbr) {
        next = grid[x-1][y]
        next += grid[x-1][y-1]
        next += grid[x-1][y+1]

        next += grid[x+1][y]
        next += grid[x+1][y-1]
        next += grid[x+1][y+1]

        next += grid[x][y-1]
        next += grid[x][y+1]

        grid[x][y] = next

        addedInLevel++

        val sideLength = level * 2
        val side = addedInLevel / sideLength

        if (side == 0) {
            y++
        } else if (side == 1) {
            x--
        } else if (side == 2) {
            y--
        } else if (side == 3) {
            x++
        }

        if (addedInLevel >= level * 8) {
            addedInLevel = 0
            level++
            x++
        }
    }
    println(next)
    grid.forEach {
        print(">")
        it.forEach { print(" ${it} ") }
        println()
    }
}

fun solve1(input: String) : Unit {
    val inputNbr = input.toInt()

    var counter = 1
    var level = 0

    while (counter < inputNbr) {
        level++
        counter += 8 * level
    }

    var steps = 0
    var at = inputNbr
    while (level != 0) {
        val pos = counter - at
        val sideLength = level * 2
        val corr: Int = pos / sideLength

        // Check the corners
        if (listOf(counter,
                counter - sideLength,
                counter - sideLength * 2,
                counter - sideLength * 3).contains(at)) {
            steps += 2
            at = at - (level * 8) + (corr * 2)
        } else {
            steps++
            at = at - (level * 8) + (corr * 2) + 1
        }
        counter -= 8 * level

        level--
    }
    println(steps)
}
