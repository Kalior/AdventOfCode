package day1

import java.io.File
import java.io.InputStream

fun main(args : Array<String>) {
    val inputStream: InputStream = File("input/day1input").inputStream()
    val inputString = inputStream.bufferedReader().use { it.readText() }

    val all = inputString.mapIndexed { index, char ->
        val nextIndex = (index + inputString.length / 2) % inputString.length
        if (inputString[nextIndex] == char) char.toInt() - '0'.toInt() else 0
    }

    val sum = all.fold(0) { total, next -> total + next}

    println(sum)
}