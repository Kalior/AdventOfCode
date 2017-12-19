package day19

import kotlin.system.measureTimeMillis

data class Input(val raw: List<String>, val map: Map<Position, String>, val start: Position)

data class Position(val x: Int, val y: Int)

fun Position.Up(): Position {
    return Position(this.x, this.y - 1)
}
fun Position.Down(): Position {
    return Position(this.x, this.y + 1)
}
fun Position.Left(): Position {
    return Position(this.x - 1, this.y)
}
fun Position.Right(): Position {
    return Position(this.x + 1, this.y)
}

enum class Direction {
    NORTH, SOUTH, WEST, EAST, NONE
}

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
    val parser = util.Parser("19")
    val lines = parser.getLines()

    val map = mutableMapOf<Position, String>()
    var y = 0
    var start = Position(0, 0)
    lines.forEach { line ->
        var x = 0
        line.forEach {
            when(it) {
                ' ' -> map.put(Position(x, y), "empty")
                in listOf('-', '|', '+') -> map.put(Position(x, y), "path")
                else -> map.put(Position(x, y), it.toString())
            }
            if (y == 0 && listOf('-', '|', '+').contains(it)) start = Position(x, y)
            x++
        }
        y++
    }

    return Input(lines, map.toMap().withDefault { "empty" }, start)
}


fun solve2(input: Input) : String {
    var currentDirection = Direction.SOUTH
    var currentPosition = input.start
    var steps = 0

    while (currentDirection != Direction.NONE) {
        steps++
        val (nextPos, nextDir) = findNextPosition(input.map, currentPosition, currentDirection)
        currentPosition = nextPos
        currentDirection = nextDir
    }

    return "$steps"
}

fun solve1(input: Input) : String {
    var currentDirection = Direction.SOUTH
    var currentPosition = input.start
    var string = ""

    while (currentDirection != Direction.NONE) {
        val pos = input.map.getValue(currentPosition)
        if (pos != "path") {
            string += pos
        }
        val (nextPos, nextDir) = findNextPosition(input.map, currentPosition, currentDirection)
        currentPosition = nextPos
        currentDirection = nextDir
    }

    return string
}

fun findNextPosition(map: Map<Position, String>, position: Position, direction: Direction): Pair<Position, Direction> {
    when (direction) {
        Direction.NORTH -> {
            return when {
                map.getValue(position.Up()) != "empty" -> Pair(position.Up(), direction)
                map.getValue(position.Right()) != "empty" -> Pair(position.Right(), Direction.EAST)
                map.getValue(position.Left()) != "empty" -> Pair(position.Left(), Direction.WEST)
                else -> Pair(position, Direction.NONE)
            }
        }
        Direction.SOUTH -> {
            return when {
                map.getValue(position.Down()) != "empty" -> Pair(position.Down(), direction)
                map.getValue(position.Right()) != "empty" -> Pair(position.Right(), Direction.EAST)
                map.getValue(position.Left()) != "empty" -> Pair(position.Left(), Direction.WEST)
                else -> Pair(position, Direction.NONE)
            }
        }
        Direction.EAST -> {
            return when {
                map.getValue(position.Right()) != "empty" -> Pair(position.Right(), direction)
                map.getValue(position.Up()) != "empty" -> Pair(position.Up(), Direction.NORTH)
                map.getValue(position.Down()) != "empty" -> Pair(position.Down(), Direction.SOUTH)
                else -> Pair(position, Direction.NONE)
            }
        }
        Direction.WEST -> {
            return when {
                map.getValue(position.Left()) != "empty" -> Pair(position.Left(), direction)
                map.getValue(position.Up()) != "empty" -> Pair(position.Up(), Direction.NORTH)
                map.getValue(position.Down()) != "empty" -> Pair(position.Down(), Direction.SOUTH)
                else -> Pair(position, Direction.NONE)
            }
        }
        Direction.NONE -> return Pair(position, Direction.NONE)
    }
}
