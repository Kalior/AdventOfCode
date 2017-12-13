package day12

import java.util.*
import kotlin.system.measureTimeMillis

data class Input(val raw: List<String>, val numbers: List<List<Int>>)


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
    val parser = util.Parser("12")
    val lines = parser.getLines()
    val num = parser.getNumbersFromLines(lines)

    val numbers = num.map { it.map(Double::toInt) }

    return Input(lines, numbers)
}


fun solve2(input: Input) : String {
    val connections = buildConnections(input.numbers)
    val allNumbers = connections.keys.toMutableList()

    var numGroups = 0
    while (allNumbers.isNotEmpty()) {
        numGroups++
        val firstEl = allNumbers[0]
        val group = getGroup(firstEl, connections)
        allNumbers.removeAll(group)
    }

    return numGroups.toString()
}

fun solve1(input: Input) : String {
    val connections = buildConnections(input.numbers)
    val group = getGroup(0, connections)
    return group.size.toString()
}

fun buildConnections(lines: List<List<Int>>): HashMap<Int, List<Int>> {
    val connections = hashMapOf<Int, List<Int>>()

    lines.forEach { line ->
        val key = line[0]

        val knownConn = connections[key] ?: listOf()
        val newConnections = knownConn.toMutableList()
        newConnections.addAll(line)

        connections.put(key, newConnections)

        line.forEach {
            val subConn = connections[it] ?: listOf()
            val newConnections = knownConn.toMutableList()
            newConnections.add(line[0])
            connections.put(it, subConn)
        }
    }
    return connections
}

fun getGroup(start: Int, connections: HashMap<Int, List<Int>>): List<Int> {
    val queue = ArrayDeque<Int>()
    queue.add(start)
    val reachable = hashSetOf(start)

    while (queue.isNotEmpty()) {
        val el = queue.pop()
        connections[el]!!.forEach {
            if (!reachable.contains(it)) {
                queue.add(it)
                reachable.add(it)
            }
        }
    }
    return reachable.toList()
}
