package day6

import kotlin.system.measureTimeMillis

data class Input(val raw: List<String>, var numbers: List<Int>)

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
    val parser = util.Parser("6")
    val lines = parser.getLines()
    val numbers = parser.getNumbersFromLines(lines)[0].map(Double::toInt)
    return Input(lines, numbers)
}

fun solve2(input: Input) : Int {
    var numbers = input.numbers.toMutableList()

    val seen = mutableMapOf<String, Int>()

    var steps = 0
    while (true) {
        val max = numbers.indices.maxBy { numbers[it] } ?: -1
        steps++
        numbers = shiftData(numbers, max)

        val key = numbers.toString()
        if (seen.contains(key)) {
            val lastSeen = seen[key] ?: 0
            val cycle = steps - lastSeen

            return cycle
        } else {
            seen.put(key, steps)
        }
    }
}

fun shiftData(numbers: MutableList<Int>, index: Int) : MutableList<Int> {
    var data = numbers[index]
    numbers[index] = 0
    val length = numbers.size
    var i = (index + 1) % length
    while (data != 0) {
        numbers[i]++
        data--
        i = (i + 1) % length
    }

    return numbers
}


fun solve1(input: Input) : Int {
    var numbers = input.numbers.toMutableList()

    val seen = mutableSetOf<String>()
    var steps = 0

    while (true) {
        val max = numbers.indices.maxBy { numbers[it] } ?: -1
        steps++
        numbers = shiftData(numbers, max)

        val key = numbers.toString()
        if (seen.contains(key)) {
            return steps
        } else {
            seen.add(key)
        }
    }
}
