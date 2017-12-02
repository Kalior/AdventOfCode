package day2

import java.io.File
import java.io.InputStream
import kotlin.system.measureTimeMillis

fun main(args : Array<String>) {
    val mil = measureTimeMillis { solve() }
    println(mil)
}

fun solve() : Unit {
    val inputStream: InputStream = File("input/day2input").inputStream()
    val lineList = mutableListOf<String>()

    inputStream.bufferedReader().useLines { lines -> lines.forEach { lineList.add(it) } }

    val floatRegex = "([+|-]?\\d+\\.?\\d*)".toRegex()

    val input = lineList.map { line ->
        floatRegex.findAll(line).map { it.groupValues[0].toDouble() }.toList()
    }

    val checksum = input.sumByDouble { rowDiff(it) }

    val betterChecksum = input.sumByDouble { rowDiv(it) }

    println(betterChecksum)
    println(checksum)
}

fun rowDiff(numbers: List<Double>): Double {
    val largest = numbers.max() ?: 0.0
    val smallest = numbers.min() ?: 0.0
    return largest - smallest
}

fun rowDiv(numbers: List<Double>): Double {
    numbers.forEach { x ->
        numbers.forEach { y ->
            if (x % y == 0.0 && x != y) {
                return x / y
            }
        }
    }
    return 0.0
}