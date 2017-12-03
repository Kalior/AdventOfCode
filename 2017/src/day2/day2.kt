package day2

import kotlin.system.measureTimeMillis

fun main(args : Array<String>) {
    val mil = measureTimeMillis {
        val input = parse()
        solve(input)
    }
    println(mil)
}

fun parse() : List<List<Double>> {
    val parser = util.Parser("2")
    val lines = parser.getLines()
    val input = parser.getNumbersFromLines(lines)
    return input
}

fun solve(input: List<List<Double>>) : Unit {
    val checksum = input.sumByDouble { rowDiff(it) }

    val betterChecksum = input.sumByDouble { rowDiv(it) }

    println(betterChecksum)
    println(checksum)
}

fun rowDiff(numbers: List<Double>): Double {
    val largest = numbers.max() ?: 0.0
    val smallest = numbers.min() ?: 0.0
    return largest - smallest
}

fun rowDiv(numbers: List<Double>): Double {
    numbers.forEach { x ->
        numbers.forEach { y ->
            if (x % y == 0.0 && x != y) {
                return x / y
            }
        }
    }
    return 0.0
}