package day4

import kotlin.system.measureTimeMillis

fun main(args : Array<String>) {
    val mil = measureTimeMillis {
        val input = parse()
        solve1(input)
        solve2(input)
    }
    println(mil)
}

fun parse() : List<String> {
    val parser = util.Parser("4")
    val lines = parser.getLines()
    return lines
}

fun solve2(input: List<String>) : Unit {
    val n = input.filter(::validPhrase2).size
    println(n)
}

fun validPhrase2(phrase: String) : Boolean {
    val words = phrase.split(" ")
    val wordSet = mutableSetOf<String>()
    words.forEach { word ->
        val sorted = word.toSortedSet().toList().toString()
        if (wordSet.contains(sorted)) {
            return false
        } else {
            wordSet.add(sorted)
        }
    }
    return true
}

fun solve1(input: List<String>) : Unit {
    val n = input.filter(::validPhrase).size
    println(n)
}

fun validPhrase(phrase: String) : Boolean {
    val words = phrase.split(" ")
    val wordSet = mutableSetOf<String>()
    words.forEach { word ->
        if (wordSet.contains(word)) {
            return false
        } else {
            wordSet.add(word)
        }
    }
    return true
}