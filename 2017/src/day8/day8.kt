package day8

import java.util.*
import kotlin.system.measureTimeMillis

data class Input(val raw: List<String>, val instructions: List<Instruction>)

data class Instruction(val reg: String, val condReg: String, val instr: (Int) -> Int, val condFun: (Int) -> Boolean)

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
    val parser = util.Parser("8")
    val lines = parser.getLines()

    val instructions = lines.map { parseInstruction(it, parser) }

    return Input(lines, instructions)
}

fun parseInstruction(line: String, parser: util.Parser): Instruction {
    val nbrRegex = "(-?\\d+)".toRegex()
    val namesRegex = "([a-zA-Z]+)".toRegex()
    val condRegex = "([<=!>]+)".toRegex()

    val ints = parser.getAllMatchesFromString(nbrRegex, line)
    val names = parser.getAllMatchesFromString(namesRegex, line)
    val conds = parser.getAllMatchesFromString(condRegex, line)

    val reg: String = names[0][0].trim()
    val condReg: String = names[3][0].trim()

    val instr: String = names[1][0].trim()
    val value: Int = ints[0][0].toInt()
    val instrFun: (Int) -> Int = when (instr) {
        "inc" -> {x -> x + value}
        "dec" -> {x -> x - value}
        else -> {x -> x}
    }

    val cond: String = conds[0][0].trim()
    val condValue: Int = ints[1][0].toInt()
    val condFun: (Int) -> Boolean = when (cond) {
        "<=" -> {x -> x <= condValue}
        ">=" -> {x -> x >= condValue}
        "==" -> {x -> x == condValue}
        "!=" -> {x -> x != condValue}
        "<" -> {x -> x < condValue}
        ">" -> {x -> x > condValue}
        else -> {x -> true}
    }

    return Instruction(reg, condReg, instrFun, condFun)
}

fun solve2(input: Input) : String {
    val registers = hashMapOf<String, Int>()
    var largestRegisterVal = 0
    input.instructions.forEach {
        val condRegVal = registers[it.condReg] ?: 0

        if (it.condFun(condRegVal)) {
            val regVal = registers[it.reg] ?: 0
            registers.put(it.reg, it.instr(regVal))
        }
        registers.forEach {
            if (it.value > largestRegisterVal) {
                largestRegisterVal = it.value
            }
        }
    }

    return largestRegisterVal.toString()
}

fun solve1(input: Input) : String {
    val registers = hashMapOf<String, Int>()
    var largestRegisterVal = 0
    input.instructions.forEach {
        val condRegVal = registers[it.condReg] ?: 0

        if (it.condFun(condRegVal)) {
            val regVal = registers[it.reg] ?: 0
            registers.put(it.reg, it.instr(regVal))
        }
    }
    registers.forEach {
        if (it.value > largestRegisterVal) {
            largestRegisterVal = it.value
        }
    }

    return largestRegisterVal.toString()
}
