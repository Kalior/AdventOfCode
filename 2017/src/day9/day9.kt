package day9

import kotlin.system.measureTimeMillis

data class Input(val raw: String)

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
    val parser = util.Parser("9")
    val lines = parser.getLines()

    return Input(lines[0])
}


fun solve2(input: Input) : String {
    val (endIndex, score, garbage) = readGroup(input.raw, 1, 1)

    return garbage.toString()
}

fun solve1(input: Input) : String {
    val (endIndex, score, garbage) = readGroup(input.raw, 1, 1)
    return score.toString()
}

fun readGroup(stream: String, start: Int, score: Int) : Triple<Int, Int, Int> {
    var totalScore = score
    var index = start
    var totalGarbageCount = 0
    while (stream[index] != '}') {
        when (stream[index]) {
            '{' -> {
                val (endIndex, groupScore, garbageCount) = readGroup(stream, index + 1, score + 1)
                index = endIndex
                totalScore += groupScore
                totalGarbageCount += garbageCount
            }
            '<' -> {
                val (endIndex, garbageCount) = readGarbage(stream, index + 1)
                index = endIndex
                totalGarbageCount += garbageCount
            }
            else -> index++
        }
    }

    return Triple(index + 1, totalScore, totalGarbageCount)
}

fun readGarbage(stream: String, start: Int) : Pair<Int, Int> {
    var index = start
    var count = 0
    while (stream[index] != '>') {
        when (stream[index]) {
            '!' -> index += 2
            else -> {
                index++
                count++
            }
        }
    }
    return Pair(index + 1, count)
}