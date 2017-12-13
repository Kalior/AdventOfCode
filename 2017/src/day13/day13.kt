package day13

import kotlin.system.measureTimeMillis

data class Input(val raw: List<String>, val layers: List<Layer>)

data class Layer(val depth: Int, val range: Int)

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
    val parser = util.Parser("13")
    val lines = parser.getLines()
    val layers = lines.map {
        val nbrs = it.split(": ")
        Layer(nbrs[0].toInt(), nbrs[1].toInt())
    }

    return Input(lines, layers)
}

fun solve2(input: Input) : String {
    var delay = 0
    var collides = true

    while (collides) {
        delay += 2
        collides = collidesWithScanners(delay, input.layers)
    }

    return delay.toString()
}

fun solve1(input: Input) : String {
    val severity = severityOfCollision(input.layers)
    return severity.toString()
}

fun severityOfCollision(layers: List<Layer>): Int {
    var severity = 0
    layers.forEach { layer ->
        val timeToReachLayer = layer.depth
        val collisionAt = ((layer.range - 1) * 2)
        if (timeToReachLayer % collisionAt == 0) {
            severity += layer.depth * layer.range
        }
    }
    return severity
}

fun collidesWithScanners(delay: Int, layers: List<Layer>): Boolean {
    layers.forEach { layer ->
        val timeToReachLayer = delay + layer.depth
        val collisionAt = ((layer.range - 1) * 2)
        if (timeToReachLayer % collisionAt == 0) {
            return true
        }
    }
    return false
}