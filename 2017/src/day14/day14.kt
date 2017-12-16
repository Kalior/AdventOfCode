package day14

import kotlin.system.measureTimeMillis

data class Input(val input: String)

data class Position(val x: Int, val y: Int)

fun Position.up(): Position {
    return Position(this.x, this.y - 1)
}
fun Position.down(): Position {
    return Position(this.x, this.y + 1)
}
fun Position.left(): Position {
    return Position(this.x - 1, this.y)
}
fun Position.right(): Position {
    return Position(this.x + 1, this.y)
}

fun Int.toHexString(): String {
    val hex =  java.lang.Integer.toHexString(this)
    return if (hex.length != 2) {
        "0" + hex
    } else {
        hex
    }
}

fun String.toBinary(): String {
    return this.map {
        when (it) {
            '0' -> "0000"
            '1' -> "0001"
            '2' -> "0010"
            '3' -> "0011"
            '4' -> "0100"
            '5' -> "0101"
            '6' -> "0110"
            '7' -> "0111"
            '8' -> "1000"
            '9' -> "1001"
            'a' -> "1010"
            'b' -> "1011"
            'c' -> "1100"
            'd' -> "1101"
            'e' -> "1110"
            'f' -> "1111"
            else -> "1111"
        }
    }.joinToString("")
}

fun main(args : Array<String>) {
    val mil = measureTimeMillis {
        val input = parse()
        val sol1 = solve1(input)
        val sol2 = solve2(input)
        println("Solution 1: $sol1")
        println("Solution 2: $sol2")
    }
    println("Time: $mil (ms)")
}

fun parse() : Input {
    val input = "xlqgujun"

    return Input(input)
}

fun solve2(input: Input) : String {
    val grid = mutableMapOf<Position, Char>().withDefault { '0' }
    (0..127).forEach { y ->
        calcRow(input.input + "-$y").forEachIndexed { x, c ->
            grid.put(Position(x, y), c)
        }
    }

    var currentGroup = 0
    val groupGrid = mutableMapOf<Position, Int>()

    for (y in (0..127)) {
        for (x in (0..127)) {
            if (!groupGrid.containsKey(Position(x, y)) && grid[Position(x, y)]!! == '1') {
                currentGroup++
                groupGrid[Position(x, y)] = currentGroup
                markEntireGroup(groupGrid, grid, currentGroup, Position(x, y))
            }
        }
    }

    return currentGroup.toString()
}

fun markEntireGroup(groupGrid: MutableMap<Position, Int>, grid: MutableMap<Position, Char>, group: Int, position:
Position) {
    if (!groupGrid.containsKey(position.up()) && grid.containsKey(position.up()) && grid[position.up()]!! == '1') {
        groupGrid[position.up()] = group
        markEntireGroup(groupGrid, grid, group, position.up())
    }
    if (!groupGrid.containsKey(position.down()) && grid.containsKey(position.down()) && grid[position.down()]!! == '1') {
        groupGrid[position.down()] = group
        markEntireGroup(groupGrid, grid, group, position.down())
    }
    if (!groupGrid.containsKey(position.right()) && grid.containsKey(position.right()) && grid[position.right()]!! == '1') {
        groupGrid[position.right()] = group
        markEntireGroup(groupGrid, grid, group, position.right())
    }
    if (!groupGrid.containsKey(position.left()) && grid.containsKey(position.left()) && grid[position.left()]!! == '1') {
        groupGrid[position.left()] = group
        markEntireGroup(groupGrid, grid, group, position.left())
    }
}

fun solve1(input: Input) : String {
    val sum = (0..127)
            .map { calcRow(input.input + "-$it")
                    .fold(0) { acc, c -> c.toString().toInt() + acc } }
            .sum()

    return sum.toString()
}


fun calcRow(input: String): String {
    val ascii = input.toCharArray().map(Char::toInt).toMutableList()
    val suffix = listOf(17, 31, 73, 47, 23)
    ascii.addAll(suffix)
    val hash = day10.calcKnotHash(ascii)
    return hash.toBinary()
}