package day18

import kotlinx.coroutines.experimental.*
import kotlinx.coroutines.experimental.channels.Channel
import kotlin.system.measureTimeMillis

data class Input(val lines: List<String>)

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
    val parser = util.Parser("18")
    val lines = parser.getLines()

    return Input(lines)
}

fun solve2(input: Input) : String {
    return runBlocking {
        val channelOne = Channel<Long>(1000)
        val channelTwo = Channel<Long>(1000)

        async { run(input.lines, 0L, channelOne, channelTwo) }
        val counterTwo = async { run(input.lines, 1L, channelTwo, channelOne) }

        "${counterTwo.await()}"
    }
}

suspend fun run(lines: List<String>, programID: Long, sendChannel: Channel<Long>, receiveChannel: Channel<Long>): Int {
    val registers = mutableMapOf<String, Long>()
    var position = 0
    val numberOfInstructions = lines.size
    var counter = 0

    registers.put("p", programID)

    while (position in 0 until numberOfInstructions) {
        val line = lines[position]
        val parts = line.split(" ")
        val step = when (parts[0]) {
            "snd" -> {counter++; sndReal(registers, parts[1], sendChannel)}
            "set" -> set(registers, parts[1], parts[2])
            "add" -> add(registers, parts[1], parts[2])
            "mul" -> mul(registers, parts[1], parts[2])
            "mod" -> mod(registers, parts[1], parts[2])
            "rcv" -> rcvReal(registers, parts[1], receiveChannel)
            "jgz" -> jgz(registers, parts[1], parts[2])
            else -> 10000
        }
        position += step
    }
    sendChannel.close()
    return counter
}

suspend fun sndReal(registers: MutableMap<String, Long>, firstArgument: String, sendChannel: Channel<Long>) : Int {
    val firstInt = firstArgument.toLongOrNull() ?: registers[firstArgument]!!


    sendChannel.send(firstInt)

    return 1
}

suspend fun rcvReal(registers: MutableMap<String, Long>, firstArgument: String, receiveChannel: Channel<Long>) : Int {
    if (receiveChannel.isClosedForReceive) return 10000

    val value = withTimeoutOrNull(1000L) { receiveChannel.receive() }
    if (value == null) {
        return 10000
    } else {
        registers.put(firstArgument, value)
    }

    return 1
}


fun solve1(input: Input) : String {
    val registers = mutableMapOf<String, Long>()

    var position = 0

    while (registers["rcv"] == null) {
        val line = input.lines[position]
        val parts = line.split(" ")

        val step = when (parts[0]) {
            "snd" -> snd(registers, parts[1])
            "set" -> set(registers, parts[1], parts[2])
            "add" -> add(registers, parts[1], parts[2])
            "mul" -> mul(registers, parts[1], parts[2])
            "mod" -> mod(registers, parts[1], parts[2])
            "rcv" -> rcv(registers, parts[1])
            "jgz" -> jgz(registers, parts[1], parts[2])
            else -> 10000
        }
        position += step
    }

    return registers["rcv"].toString()
}

fun snd(registers: MutableMap<String, Long>, firstArgument: String) : Int {
    val firstInt = firstArgument.toLongOrNull() ?: registers[firstArgument]!!

    registers.put("snd", firstInt)

    return 1
}

fun rcv(registers: MutableMap<String, Long>, firstArgument: String) : Int {
    val key = firstArgument.toLongOrNull() ?: registers[firstArgument]!!

    if (key != 0.toLong()) {
        val snd = registers.remove("snd")!!
        registers.put("rcv", snd)
    }
    return 1
}

fun set(registers: MutableMap<String, Long>, firstArgument: String, secondArgument: String) : Int {
    val value = secondArgument.toLongOrNull() ?: registers[secondArgument]!!

    registers.put(firstArgument, value)
    return 1

}
fun add(registers: MutableMap<String, Long>, firstArgument: String, secondArgument: String) : Int {
    val value = secondArgument.toLongOrNull() ?: registers[secondArgument]!!

    val regValue = registers[firstArgument] ?: 0
    registers.put(firstArgument, regValue + value)
    return 1
}
fun mul(registers: MutableMap<String, Long>, firstArgument: String, secondArgument: String) : Int {
    val value = secondArgument.toLongOrNull() ?: registers[secondArgument]!!

    val regValue = registers[firstArgument] ?: 0
    registers.put(firstArgument, regValue * value)
    return 1
}
fun mod(registers: MutableMap<String, Long>, firstArgument: String, secondArgument: String) : Int {
    val value = secondArgument.toLongOrNull() ?: registers[secondArgument]!!

    val regValue = registers[firstArgument] ?: 0
    registers.put(firstArgument, regValue % value)
    return 1
}
fun jgz(registers: MutableMap<String, Long>, firstArgument: String, secondArgument: String) : Int {

    val firstValue = firstArgument.toLongOrNull() ?: registers[firstArgument]!!

    val secondValue = secondArgument.toLongOrNull() ?: registers[secondArgument]!!

    if (firstValue > 0L) {
        return secondValue.toInt()
    }
    return 1
}