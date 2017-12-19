package day17

import kotlin.system.measureTimeMillis

data class Input(val number: Int)

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
    val parser = util.Parser("17")
    val lines = parser.getLines()
    val number = lines[0].toInt()

    return Input(number)
}

fun solve2(input: Input) : String {
    var position = 0
    var bufferSize = 1
    var lastValueAfterZero = 0
    for (i in (0 until 50000000)) {
        position = fakeInsertion(position, input.number, bufferSize) + 1

        if (position == 1) {
            lastValueAfterZero = i + 1
        }
        bufferSize++
    }

    return lastValueAfterZero.toString()
}

fun solve1(input: Input) : String {
    var buffer = listOf(0)

    var position = 0
    for (i in (0 until 2017)) {
        val (newBuffer, newPosition) = insertion(buffer, position, input.number, i + 1)
        position = newPosition + 1
        buffer = newBuffer
    }

    return buffer[position + 1].toString()
}

fun insertion(buffer: List<Int>, startPosition: Int, steps: Int, input: Int) : Pair<List<Int>, Int> {
    val insertPosition = (startPosition + steps) % buffer.size
    val firstPart = buffer.slice((0 .. insertPosition))
    val secondPart = buffer.slice((insertPosition + 1 until buffer.size))

    val newBuffer = mutableListOf<Int>()
    newBuffer.addAll(firstPart)
    newBuffer.add(input)
    newBuffer.addAll(secondPart)
    return Pair(newBuffer, insertPosition)
}

fun fakeInsertion(startPosition: Int, steps: Int, bufferSize: Int) : Int {

    return (startPosition + steps) % bufferSize
}