package day16

import kotlin.system.measureTimeMillis

data class Input(val moves: List<Move>)

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
    val parser = util.Parser("16")
    val lines = parser.getLines()
    val moves = lines[0].split(",")

    val parsedMoves = moves.map {
        when (it[0]) {
            's' -> parseSpin(it)
            'x' -> parseExchange(it)
            'p' -> parsePartner(it)
            else -> Spin(0)
        }
    }

    return Input(parsedMoves)
}

fun solve2(input: Input) : String {
    val programs  = mutableListOf('a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p')

    val s = programs.joinToString("")
    for (i in (0 until 1000000000 % 36)) {
        input.moves.forEach { it.act(programs) }
        if (programs.joinToString("") == s) {
            println(i)
        }
    }

    return programs.joinToString("")
}

fun solve1(input: Input) : String {
    val programs  = mutableListOf('a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p')

    input.moves.forEach { it.act(programs) }

    return programs.joinToString("")
}
