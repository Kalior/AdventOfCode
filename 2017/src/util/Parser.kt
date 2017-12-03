package util

import java.io.File
import java.io.InputStream

class Parser(val day: String) {
    fun getLines() : List<String> {
        val inputStream: InputStream = File("input/day${day}input").inputStream()
        val lineList = mutableListOf<String>()

        inputStream.bufferedReader().useLines { lines -> lines.forEach { lineList.add(it) } }
        return lineList
    }

    fun getNumbersFromLines(input: List<String>): List<List<Double>> {
        val floatRegex = "([+|-]?\\d+\\.?\\d*)".toRegex()
        val matches = getFromLines(floatRegex, input)

        val numbers = matches.map { line -> line.map { it[0].toDouble() } }
        return numbers
    }

    fun getFromLines(regex: Regex, input: List<String>): List<List<List<String>>> {
        val matches = input.map { getAllMatchesFromString(regex, it) }
        return matches
    }

    fun getAllMatchesFromString(regex: Regex, line: String) : List<List<String>> {
        val matches = regex.findAll(line).map { it.groupValues.toList() }.toList()
        return matches
    }
}