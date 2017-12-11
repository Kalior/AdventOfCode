package day11

import kotlin.system.measureTimeMillis

data class Input(val raw: String, val directions: List<String>)


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
    val parser = util.Parser("11")
    val lines = parser.getLines()
    val directions = lines[0].split(",")

    return Input(lines[0], directions)
}


fun solve2(input: Input) : String {
    var x = 0
    var y = 0
    var z = 0
    var steps = 0
    for (dir in input.directions) {
        when (dir) {
            "se" -> {x--; z++; steps = Math.max(distance(x, y, z), steps)}
            "sw" -> {y++; z--; steps = Math.max(distance(x, y, z), steps)}
            "ne" -> {y--; z++; steps = Math.max(distance(x, y, z), steps)}
            "nw" -> {x++; z--; steps = Math.max(distance(x, y, z), steps)}
            "s" -> {y++; x--; steps = Math.max(distance(x, y, z), steps)}
            "n" -> {y--; x++; steps = Math.max(distance(x, y, z), steps)}
        }
    }

    return "$steps"
}

fun solve1(input: Input) : String {
    var x = 0
    var y = 0
    var z = 0
    for (dir in input.directions) {
        when (dir) {
            "se" -> {x--; z++}
            "sw" -> {y++; z--}
            "ne" -> {y--; z++}
            "nw" -> {x++; z--}
            "s" -> {y++; x--}
            "n" -> {y--; x++}
        }
    }
    val steps = distance(x, y, z)

    return "$steps"
}

fun distance(x: Int, y: Int, z: Int): Int {
    return (Math.abs(x) + Math.abs(y) + Math.abs(z)) / 2
}










