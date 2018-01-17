package day21

import kotlin.system.measureTimeMillis

data class Input(val raw: List<String>, val rules: List<Rule>)

data class Rule(val grid: List<List<Int>>, val result: List<List<Int>>) {
    fun matches(grid: List<List<Int>>): Boolean =
            grid == this.grid ||
            grid.rotate() == this.grid ||
            grid.rotate().asReversed() == this.grid ||
            grid.rotate().rotate() == this.grid ||
            grid.rotate().rotate().asReversed() == this.grid ||
            grid.rotate().rotate().rotate() == this.grid ||
            grid.rotate().rotate().rotate().asReversed() == this.grid
}

fun<E> List<List<E>>.rotate(): List<List<E>> {
    return this.asTransposed().asReversed()
}

fun<E> List<List<E>>.asTransposed(): List<List<E>> {
    if (this.isEmpty() || this[0].isEmpty()) {
        return listOf()
    }

    val transposed = mutableListOf<List<E>>()

    (0 until this[0].size).forEach { j ->
        val innerList = mutableListOf<E>()
        (0 until this.size).forEach { i -> innerList.add(this[i][j]) }
        transposed.add(innerList.toList())
    }

    return transposed.toList()
}

fun main(args: Array<String>) {
    val mil = measureTimeMillis {
        val input = parse()
        val sol1 = solve1(input)
        val sol2 = solve2(input)
        println("Solution 1: $sol1")
        println("Solution 2: $sol2")
    }
    println("Time: $mil (ms)")
}

fun parse(): Input {
    val parser = util.Parser("21")
    val lines = parser.getLines()
    val rules = lines.map { parseRule(it) }

    return Input(lines, rules)
}

fun parseRule(line: String): Rule {
    val (gridString, resultString) = line.split(" => ")
    val grid = parseStringToPattern(gridString)
    val result = parseStringToPattern(resultString)

    return Rule(grid, result)
}

fun parseStringToPattern(string: String): List<List<Int>> {
    val rows = string.split("/")

    return rows.map { it.toCharArray().map { charToValue(it) } }
}

fun charToValue(c: Char): Int = when (c) {
    '.' -> 0
    '#' -> 1
    else -> 0
}

fun solve2(input: Input): String {
    var grid = listOf(listOf(0, 1, 0), listOf(0, 0, 1), listOf(1, 1, 1))
    (0 until 18).forEach { grid = expandSquares(grid, input.rules) }

    val sum = grid.sumBy { it.sum() }
    return sum.toString()
}

fun solve1(input: Input): String {
    var grid = listOf(listOf(0, 1, 0), listOf(0, 0, 1), listOf(1, 1, 1))
    (0 until 5).forEach { grid = expandSquares(grid, input.rules) }

    val sum = grid.sumBy { it.sum() }
    return sum.toString()
}

fun expandSquares(grid: List<List<Int>>, rules: List<Rule>): List<List<Int>> {
     val size = if (grid.size % 2 == 0) {
        2
    } else {
        3
    }
    val allSquares = getAllSquares(size, grid)
    val expandedSquares = allSquares.map { expandSquare(it, rules) }
    val expandedGrid = joinSquares(expandedSquares)
    return expandedGrid
}

fun joinSquares(squares: List<List<List<Int>>>): List<List<Int>> {
    val size = squares[0].size
    val numberOfRows = Math.sqrt(squares.size.toDouble()).toInt()

    val grid = (0 until numberOfRows * size).map { mutableListOf<Int>() }.toMutableList()

    for (i in (0 until squares.size)) {
        val offset = (i % numberOfRows) * size
        squares[i].withIndex().forEach { grid[offset + it.index].addAll(it.value) }
    }

    return grid
}

fun getAllSquares(size: Int, grid: List<List<Int>>): List<List<List<Int>>> {
    return (0 until grid.size / size).flatMap {
        x -> (0 until grid.size / size).map {
            y -> getSquare(size, x * size, y * size, grid)
        }
    }
}

fun expandSquare(square: List<List<Int>>, rules: List<Rule>): List<List<Int>> {
    val rule = rules.find { it.matches(square) }!!
    return rule.result
}

fun getSquare(size: Int, x: Int, y: Int, grid: List<List<Int>>): List<List<Int>> {
    return (y until y + size).map { grid[it].slice((x until x + size)) }
}