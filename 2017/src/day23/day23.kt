package day23

import kotlin.system.measureTimeMillis

data class Input(val lines: List<String>)

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
    val parser = util.Parser("23")
    val lines = parser.getLines()

    return Input(lines)
}

fun solve2(input: Input): String {
    val registers = mutableMapOf<String, Long>().withDefault { 0 }
    registers.put("a", 1)
    // "b" = 81 * 100 + 100000 = 108100
    // "c" = "b" + 17000 = 125100
    registers.put("b", 108100)
    registers.put("c", 125100)

    var block = 1
    while (block != -1) {
        block = when (block) {
            1 -> block1(registers)
            2 -> block2(registers)
            3 -> block3(registers)
            4 -> block4(registers)
            5 -> block5(registers)
            else -> -1
        }
    }

    return registers.getValue("h").toString()
}

fun block1(registers: MutableMap<String, Long>): Int {
    registers.put("f", 1)
    registers.put("d", 2)
    return 2
}

fun block2(registers: MutableMap<String, Long>): Int {
    registers.put("e", 2)
    return 3
}

fun block3(registers: MutableMap<String, Long>): Int {
    val d = registers.getValue("d")
    var e = registers.getValue("e")
    val b = registers.getValue("b")

    do {
        if (d * e - b == 0L) {
            registers.put("f", 0)
        }
        e++
        registers.put("e", e)

    } while (e - b != 0L)

    return 4

}

fun block4(registers: MutableMap<String, Long>): Int {
    val b = registers.getValue("b")
    var d = registers.getValue("d")

    do {
        d++
        registers.put("d", d)

        registers.put("e", b)

        val div = b.toFloat() / d.toFloat()
        if (div.toInt().toFloat() == div && div in (2 .. b)) {
            registers.put("f", 0)
        }

    } while (d - b != 0L)

    val f = registers.getValue("f")
    return when {
        f != 0L -> {
            println("$registers, block4 f != 0")
            5
        }
        else -> {
            println("$registers, about to increase h")
            val h = registers.getValue("h")
            registers.put("h", h + 1)
            5
        }
    }
}

fun block5(registers: MutableMap<String, Long>): Int {
    val b = registers.getValue("b")
    val c = registers.getValue("c")
    registers.put("g", b - c)

    return if (b - c == 0L) {
        -1
    } else {
        registers.put("b", b + 17)
        1
    }
}

fun solve1(input: Input): String {
    val registers = mutableMapOf<String, Long>().withDefault { 0 }

    var position = 0
    var counter = 0
    val numberOfInstructions = input.lines.size
    while (position in 0 until numberOfInstructions) {
        val line = input.lines[position]
        val parts = line.split(" ")

        val step = when (parts[0]) {
            "set" -> set(registers, parts[1], parts[2])
            "sub" -> sub(registers, parts[1], parts[2])
            "mul" -> {counter++; mul(registers, parts[1], parts[2])}
            "jnz" -> jnz(registers, parts[1], parts[2])
            else -> 10000
        }

        position += step
    }

    return counter.toString()
}

fun set(registers: MutableMap<String, Long>, firstArgument: String, secondArgument: String) : Int {
    val value = secondArgument.toLongOrNull() ?: registers.getValue(secondArgument)

    registers.put(firstArgument, value)
    return 1

}
fun sub(registers: MutableMap<String, Long>, firstArgument: String, secondArgument: String) : Int {
    val value = secondArgument.toLongOrNull() ?: registers.getValue(secondArgument)

    val regValue = registers[firstArgument] ?: 0
    registers.put(firstArgument, regValue - value)
    return 1
}
fun mul(registers: MutableMap<String, Long>, firstArgument: String, secondArgument: String) : Int {
    val value = secondArgument.toLongOrNull() ?: registers.getValue(secondArgument)

    val regValue = registers[firstArgument] ?: 0
    registers.put(firstArgument, regValue * value)
    return 1
}
fun jnz(registers: MutableMap<String, Long>, firstArgument: String, secondArgument: String) : Int {

    val firstValue = firstArgument.toLongOrNull() ?: registers.getValue(firstArgument)

    val secondValue = secondArgument.toLongOrNull() ?: registers.getValue(secondArgument)

    if (firstValue != 0L) {
        return secondValue.toInt()
    }
    return 1
}
