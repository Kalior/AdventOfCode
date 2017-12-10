package day10

import kotlin.system.measureTimeMillis

data class Input(val raw: String, val numbers: List<Int>, val ascii: List<Int>)

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
    val parser = util.Parser("10")
    val lines = parser.getLines()
    val nbrRegex = "(-?\\d+)".toRegex()
    val ints = parser.getAllMatchesFromString(nbrRegex, lines[0])
    val numbers = ints.map { it[0].toInt() }

    val ascii = lines[0].toCharArray().map(Char::toInt).toMutableList()
    val suffix = listOf(17, 31, 73, 47, 23)
    ascii.addAll(suffix)

    return Input(lines[0], numbers, ascii)
}


fun solve2(input: Input) : String {
    var position = 0
    var skipLength = 0
    val list = (0..255).toMutableList()
    for (i in (0..63)) {
        val (newPos, newSkip) = round(position, skipLength, input.ascii, list)
        position = newPos
        skipLength = newSkip
    }

    val denseHash = calcDenseHash(list)
    val hex = denseToHex(denseHash)

    return hex
}

fun calcDenseHash(sparseHash: List<Int>): List<Int> {
    return sparseHash
            .withIndex()
            .groupBy { it.index / 16 }
            .map { it.value.fold(0) {acc, item -> acc xor item.value } }
}

fun denseToHex(denseHash: List<Int>): String {
    val hex = denseHash.map { java.lang.Integer.toHexString(it) }
            .map {
                if (it.length != 2) {
                   "0" + it
                } else {
                    it
                }
            }
    return hex.joinToString("")
}

fun solve1(input: Input) : String {
    val skipLength = 0
    val position = 0
    val list = (0..255).toMutableList()

    round(position, skipLength, input.numbers, list)

    return (list[0] * list[1]).toString()
}

fun round(initPosition: Int, initSkipLength: Int, lengths: List<Int>, list: MutableList<Int>) : Pair<Int, Int> {
    var position = initPosition
    var skipLength = initSkipLength

    lengths.forEach {
        if (it <= list.size) {
            val subList = getSubList(list, it, position)
            insertSubList(list, subList, position)
            position = (position + skipLength + it) % list.size
            skipLength++
        }
    }
    return Pair(position, skipLength)
}

fun getSubList(list: List<Int>, length: Int, position: Int) : List<Int> {
    val listLength = list.size
    val subList = (0..length-1).map { list[(position + it) % listLength] }

    return subList.asReversed()
}

fun insertSubList(list: MutableList<Int>, subList: List<Int>, position: Int) : MutableList<Int> {
    val listLength = list.size

    for (i in (0..subList.size-1)) {
        list[(position + i) % listLength] = subList[i]
    }

    return list
}















