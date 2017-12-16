package day15

import kotlin.system.measureTimeMillis

data class Input(val input: String)

fun Int.toHexString(): String {
    val hex =  java.lang.Integer.toHexString(this)
    return if (hex.length != 2) {
        "0" + hex
    } else {
        hex
    }
}

fun String.toBinary(): String {
    return this.map {
        when (it) {
            '0' -> "0000"
            '1' -> "0001"
            '2' -> "0010"
            '3' -> "0011"
            '4' -> "0100"
            '5' -> "0101"
            '6' -> "0110"
            '7' -> "0111"
            '8' -> "1000"
            '9' -> "1001"
            'a' -> "1010"
            'b' -> "1011"
            'c' -> "1100"
            'd' -> "1101"
            'e' -> "1110"
            'f' -> "1111"
            else -> "false"
        }
    }.joinToString("")
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
    val parser = util.Parser("15")
    val lines = parser.getLines()

    return Input(lines[0])
}

fun solve2(input: Input) : String {
    val genAMultiply = 16807
    val genBMultiply = 48271
    val divide = 2147483647

    var genAPrev = 591
    var genBPrev = 393

    var matches = 0

    for (i in (0 until 5000000)) {
        do  {
            genAPrev = generate(genAMultiply, divide, genAPrev)
        } while (genAPrev % 4 != 0)

        do {
            genBPrev = generate(genBMultiply, divide, genBPrev)
        } while (genBPrev % 8 != 0)

        if (compare(genAPrev, genBPrev)) {
            matches++
        }
    }

    return matches.toString()
}

fun solve1(input: Input) : String {
    val genAMultiply = 16807
    val genBMultiply = 48271
    val divide = 2147483647

    var genAPrev = 591
    var genBPrev = 393

    var matches = 0

    for (i in (0 until 40000000)) {
        genAPrev = generate(genAMultiply, divide, genAPrev)
        genBPrev = generate(genBMultiply, divide, genBPrev)

        if (compare(genAPrev, genBPrev)) {
            matches++
        }
    }

    return matches.toString()
}

fun generate(multiply: Int, divide: Int, previous: Int) : Int {
    val long = ((previous.toLong() % divide) * (multiply % divide)) % divide
    return long.toInt()

}

fun compare(val1: Int, val2: Int): Boolean {
    val bin1 = val1.toHexString().toBinary()
    val bin2 = val2.toHexString().toBinary()

    return if (bin1.length > 16 && bin2.length > 16) {
        bin1.slice((bin1.length-16 until bin1.length)) == bin2.slice((bin2.length-16 until bin2.length))
    } else {
        false
    }
}
