package day25

import kotlin.system.measureTimeMillis

data class Input(val numberOfSteps: Int, val states: List<State>)

data class State(val operation: (Int, MutableMap<Int, Value>) -> Pair<Int, Int>)

enum class Value {
    Zero, One
}

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
    val states = listOf(
            State({i, tape -> when (tape.getValue(i)) {
                Value.Zero -> { tape.put(i, Value.One); Pair(i+1, 1) }
                Value.One -> { tape.put(i, Value.One); Pair(i-1, 4) }
                null -> Pair(i, -1)
            }}),
            State({i, tape -> when (tape.getValue(i)) {
                Value.Zero -> { tape.put(i, Value.One); Pair(i+1, 2) }
                Value.One -> { tape.put(i, Value.One); Pair(i+1, 5) }
                null -> Pair(i, -1)
            }}),
            State({i, tape -> when (tape.getValue(i)) {
                Value.Zero -> { tape.put(i, Value.One); Pair(i-1, 3) }
                Value.One -> { tape.put(i, Value.Zero); Pair(i+1, 1) }
                null -> Pair(i, -1)
            }}),
            State({i, tape -> when (tape.getValue(i)) {
                Value.Zero -> { tape.put(i, Value.One); Pair(i+1, 4) }
                Value.One -> { tape.put(i, Value.Zero); Pair(i-1, 2) }
                null -> Pair(i, -1)
            }}),
            State({i, tape -> when (tape.getValue(i)) {
                Value.Zero -> { tape.put(i, Value.One); Pair(i-1, 0) }
                Value.One -> { tape.put(i, Value.Zero); Pair(i+1, 3) }
                null -> Pair(i, -1)
            }}),
            State({i, tape -> when (tape.getValue(i)) {
                Value.Zero -> { tape.put(i, Value.One); Pair(i+1, 0) }
                Value.One -> { tape.put(i, Value.One); Pair(i+1, 2) }
                null -> Pair(i, -1)
            }})
    )

    return Input(12523873, states)
}

fun solve2(input: Input): String {
    return ""
}


fun solve1(input: Input): String {
    val tape = mutableMapOf<Int, Value>().withDefault { Value.Zero }
    var cursor = 0
    var currentState = 0

    repeat(input.numberOfSteps) {
        val (newCursor, newState) = input.states[currentState].operation(cursor, tape)
        cursor = newCursor
        currentState = newState
    }

    val checksum = tape.values.fold(0) {acc, v -> if (v == Value.One) acc + 1 else acc}
    return "$checksum"
}
