package day5

import kotlin.system.measureTimeMillis

data class Input(val raw: List<String>, var numbers: List<Int>)

fun main(args : Array<String>) {
    val mil = measureTimeMillis {
        val input = parse()
        val sol1 = solve1(input)
        val sol2 = solve2(input)
        println("Solution 1: ${sol1}")
        println("Solution 2: ${sol2}")
    }
    println("Time: ${mil} (ms)")
}

fun parse() : Input {
    val parser = util.Parser("5")
    val lines = parser.getLines()
    val numbers = lines.map(String::toInt)
    return Input(lines, numbers)
}

fun solve2(input: Input) : Int {
    var pos = 0
    var steps = 0
    val numbers = input.numbers.toMutableList()
    while (pos < numbers.size) {
        val jump = numbers[pos]
        if (jump >= 3) {
            numbers[pos] -= 1
        } else {
            numbers[pos] += 1
        }

        pos += jump
        steps++
    }
    return steps
}

fun solve1(input: Input) : Int {
    var pos = 0
    var steps = 0
    val numbers = input.numbers.toMutableList()
    while (pos < numbers.size) {
        val jump = numbers[pos]
        numbers[pos] += 1

        pos += jump
        steps++
    }
    return steps
}
