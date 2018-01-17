package day22

import kotlin.system.measureTimeMillis

data class Input(val raw: List<String>, val map: Map<Position, Status>, val start: Position)

data class Position(val x: Int, val y: Int)

fun Position.forward(dir: Direction): Position = when (dir) {
    Direction.WEST -> Position(x - 1, y)
    Direction.EAST -> Position(x + 1, y)
    Direction.SOUTH -> Position(x, y + 1)
    Direction.NORTH -> Position(x, y - 1)
    Direction.NONE -> Position(x, y)
}

data class Carrier(var position: Position, var direction: Direction)

enum class Direction {
    NORTH, SOUTH, WEST, EAST, NONE
}

fun Direction.right(): Direction = when (this) {
        Direction.EAST -> Direction.SOUTH
        Direction.SOUTH -> Direction.WEST
        Direction.WEST -> Direction.NORTH
        Direction.NORTH -> Direction.EAST
        Direction.NONE -> Direction.NONE
    }

fun Direction.left(): Direction = when (this) {
        Direction.EAST -> Direction.NORTH
        Direction.SOUTH -> Direction.EAST
        Direction.WEST -> Direction.SOUTH
        Direction.NORTH -> Direction.WEST
        Direction.NONE -> Direction.NONE
    }

enum class Status {
    CLEAN, WEAKENED, INFECTED, FLAGGED
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
    val parser = util.Parser("22")
    val lines = parser.getLines()
    val (map, start) = parseMap(lines)

    return Input(lines, map, start)
}

fun parseMap(lines: List<String>): Pair<Map<Position, Status>, Position> {
    val map = mutableMapOf<Position, Status>()
    lines.withIndex().forEach { (y, v) ->
        v.withIndex().forEach { (x, c) ->
            when (c) {
                '.' -> map.put(Position(x, y), Status.CLEAN)
                '#' -> map.put(Position(x, y), Status.INFECTED)
            }

        }
    }

    val startY = lines.size / 2
    val startX = lines[0].length / 2

    return Pair(map, Position(startX, startY))
}

fun solve2(input: Input): String {
    val carrier = Carrier(input.start, Direction.NORTH)
    var counter = 0
    val map = input.map.toMutableMap().withDefault { Status.CLEAN }
    repeat(10000000) {
        val added = step2(map, carrier)
        counter += added
    }

    return counter.toString()
}

fun step2(map: MutableMap<Position, Status>, carrier: Carrier): Int {
    val infectionStatus = map.getValue(carrier.position)
    val newDirection = when (infectionStatus) {
        Status.CLEAN -> {
            map.put(carrier.position, Status.WEAKENED)
            carrier.direction.left()
        }
        Status.WEAKENED -> {
            map.put(carrier.position, Status.INFECTED)
            carrier.direction
        }
        Status.INFECTED -> {
            map.put(carrier.position, Status.FLAGGED)
            carrier.direction.right()
        }
        Status.FLAGGED -> {
            map.put(carrier.position, Status.CLEAN)
            carrier.direction.right().right()
        }

    }
    val newPosition = carrier.position.forward(newDirection)

    carrier.position = newPosition
    carrier.direction = newDirection

    return if (infectionStatus == Status.WEAKENED) 1 else 0
}


fun solve1(input: Input): String {
    val carrier = Carrier(input.start, Direction.NORTH)
    var counter = 0
    val map = input.map.toMutableMap().withDefault { Status.CLEAN }
    repeat(10000) {
        val added = step(map, carrier)
        counter += added
    }

    return counter.toString()
}

fun step(map: MutableMap<Position, Status>, carrier: Carrier): Int {
    val infectionStatus = map.getValue(carrier.position)
    val newDirection = when (infectionStatus) {
        Status.INFECTED -> {map.put(carrier.position, Status.CLEAN); carrier.direction.right()}
        Status.CLEAN -> {map.put(carrier.position, Status.INFECTED); carrier.direction.left()}
        else -> null
    }!!
    val newPosition = carrier.position.forward(newDirection)

    carrier.position = newPosition
    carrier.direction = newDirection
    return if (infectionStatus == Status.CLEAN) 1 else 0
}
