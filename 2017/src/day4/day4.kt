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
    val n = input.filter { validPhrase2(it) }.size
    println(n)
}

fun validPhrase2(phrase: String) : Boolean {
    val words = phrase.split(" ")
    val wordSet = mutableSetOf<String>()
    words.forEach { word ->
        if (wordSet.contains(word) || wordSet.any { isAnagram(word, it) }) {
            return false
        } else {
            wordSet.add(word)
        }
    }
    return true
}

fun isAnagram(s1: String, s2: String) : Boolean {
    val wordMap = mutableMapOf<Char, Int>()
    s1.forEach {
        var v = wordMap[it] ?: 0
        v++
        wordMap.put(it, v)
    }
    s2.forEach {
        var v = wordMap[it] ?: 0
        v--
        wordMap.put(it, v)
    }

    wordMap.forEach { entry ->
        if (entry.value != 0) {
            return false
        }
    }

    return true
}

fun solve1(input: List<String>) : Unit {
    val n = input.filter { validPhrase(it) }.size
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