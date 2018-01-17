package day24

import java.util.*
import kotlin.system.measureTimeMillis

data class Input(val lines: List<String>, val components: List<Component>)

data class Consider(val component: Pair<Component, Int>, val busy: BusyPort, val components: List<Component>)

data class Component(val portA: Int, val portB: Int) {

    fun findAllCompatible(components: List<Component>, with: BusyPort): List<Component> = when (with) {
        BusyPort.PortA -> components.filter { it.portA == portA || it.portB == portA }
        BusyPort.PortB -> components.filter { it.portB == portB || it.portA == portB }
    }

    fun getBusyPort(busy: BusyPort): Int = when (busy) {
        BusyPort.PortA -> portA
        BusyPort.PortB -> portB
    }

    companion object {
        fun parse(line: String): Component {
            val parts = line.split("/")
            return Component(parts[0].toInt(), parts[1].toInt())
        }
    }
}

enum class BusyPort {
    PortA, PortB
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
    val parser = util.Parser("24")
    val lines = parser.getLines()
    val components = lines.map { Component.parse(it) }

    return Input(lines, components)
}

fun solve2(input: Input): String {
    val tree = createTree2(input.components)
    val longestBridge = tree.maxBy { it.third }!!
    val bestBride = tree.filter { it.third == longestBridge.third }.maxBy { it.second }

    return "$bestBride"
}


fun createTree2(components: List<Component>): List<Triple<Component, Int, Int>> {
    val startComponent = Component(0, 0)
    val tree = mutableListOf(Triple(startComponent, 0, 0))

    val queue = ArrayDeque<Consider>()
    queue.add(Consider(Pair(startComponent, 0), BusyPort.PortA, listOf()))

    while (queue.isNotEmpty()) {
        val consider = queue.pop()
        val parentComponent = consider.component.first
        val notUsed = components.filterNot { consider.components.contains(it) }
        val compatible = parentComponent.findAllCompatible(notUsed, consider.busy)

        compatible.forEach {
            val value = consider.component.second + it.portA + it.portB
            val pair = Pair(it, value)
            val newComponents = consider.components.toMutableList()
            newComponents.add(it)
            val triple = Triple(it, value, newComponents.size)
            tree.add(triple)
            when {
                it.portA == parentComponent.getBusyPort(consider.busy) -> {
                    queue.add(Consider(pair, BusyPort.PortB, newComponents))
                }
                it.portB == parentComponent.getBusyPort(consider.busy) -> {
                    queue.add(Consider(pair, BusyPort.PortA, newComponents))
                }
            }
        }
    }
    return tree
}


fun solve1(input: Input): String {
    val tree = createTree(input.components)
    val highestValue = tree.maxBy { it.second }

    return "$highestValue"
}

fun createTree(components: List<Component>): List<Pair<Component, Int>> {
    val startComponent = Component(0, 0)
    val tree = mutableListOf(Pair(startComponent, 0))

    val queue = ArrayDeque<Consider>()
    queue.add(Consider(Pair(startComponent, 0), BusyPort.PortA, listOf()))

    while (queue.isNotEmpty()) {
        val consider = queue.pop()
        val parentComponent = consider.component.first
        val notUsed = components.filterNot { consider.components.contains(it) }
        val compatible = parentComponent.findAllCompatible(notUsed, consider.busy)

        compatible.forEach {
            val value = consider.component.second + it.portA + it.portB
            val pair = Pair(it, value)
            tree.add(pair)
            val newComponents = consider.components.toMutableList()
            newComponents.add(it)
            when {
                it.portA == parentComponent.getBusyPort(consider.busy) -> {
                    queue.add(Consider(pair, BusyPort.PortB, newComponents))
                }
                it.portB == parentComponent.getBusyPort(consider.busy) -> {
                    queue.add(Consider(pair, BusyPort.PortA, newComponents))
                }
            }
        }
    }
    return tree
}
