package day7

import com.sun.org.apache.xpath.internal.operations.Bool
import java.util.*
import kotlin.system.measureTimeMillis

data class Input(val raw: List<String>, val nodes: List<Node>)

data class Node(val weight: Int, val children: List<String>, val name: String)

fun main(args : Array<String>) {
    val mil = measureTimeMillis {
        val input = parse()
        val sol1 = solve1(input)
        val sol2 = solve2(input, sol1)
        println("Solution 1: $sol1")
        println("Solution 2: $sol2")
    }
    println("Time: $mil (ms)")
}

fun parse() : Input {
    val parser = util.Parser("7")
    val lines = parser.getLines()

    val nodes = lines.map { parseNode(it, parser) }

    return Input(lines, nodes)
}

fun parseNode(line: String, parser: util.Parser) : Node {
    val nbrRegex = "(\\d+)".toRegex()
    val namesRegex = "([a-zA-Z]+)\\s?".toRegex()
    val weights = parser.getAllMatchesFromString(nbrRegex, line)
    val weight = weights[0][0].toInt()
    val names = parser.getAllMatchesFromString(namesRegex, line)

    val nameList = mutableListOf<String>()
    val name: String = names[0][0].trim()

    names.map { nameList.add(it[0].trim()) }
    nameList.removeAt(0)

    return Node(weight, nameList, name)
}

fun solve2(input: Input, root: String) : Int {
    val nodeMap = hashMapOf<String, Node>()
    input.nodes.forEach { nodeMap.put(it.name, it) }

    val weightMap = hashMapOf<String, Int>()
    input.nodes.forEach { weightMap.put(it.name, calcWeight(nodeMap, it.name)) }

    val weight = findImbalance(nodeMap, weightMap, root)

    return weight
}

fun findImbalance(nodeMap: HashMap<String, Node>, weightMap: HashMap<String, Int>, node: String): Int {
    val queue = ArrayDeque<Pair<String, String>>()
    queue.add(Pair(node, ""))
    while (queue.isNotEmpty()) {
        val (current, parent) = queue.pop()
        val children = nodeMap[current]!!.children

        if (isBalanced(nodeMap, weightMap, current)) { // If I'm balanced, I may be the problem :/
            val siblings = nodeMap[parent]!!.children

            // If every sibling either is me or have different weight than I do
            //  then I'm the imbalanced one :/
            if (siblings.all { it == current || weightMap[it] != weightMap[current]}) {
                siblings.forEach {
                    if (it != current) {
                        val diff = weightMap[current]!! - weightMap[it]!!
                        return nodeMap[current]!!.weight - diff
                    }
                }
            }
        } else { // If I'm not balanced, one of my children def is the problem
            children.map { queue.add(Pair(it, current)) }
        }
    }

    return -1
}

fun isBalanced(nodeMap: HashMap<String, Node>, weightMap: HashMap<String, Int>, node: String) : Boolean {
    val weights = nodeMap[node]!!.children.map { weightMap[it]!! }

    return weights.isEmpty() || weights.all { weights[0] == it }
}


fun calcWeight(nodeMap: HashMap<String, Node>, current: String): Int {
    val my = nodeMap[current]!!.weight
    val children = nodeMap[current]!!.children.map { calcWeight(nodeMap, it) }.sum()

    return my + children
}

fun solve1(input: Input) : String {
    val nodeMap = hashMapOf<String, Node>()
    input.nodes.map { nodeMap.put(it.name, it) }

    val root = goUp(nodeMap, input.nodes[0].name)

    return root
}

fun goUp(nodeMap: HashMap<String, Node>, current: String): String {
    val parent = findParent(nodeMap, current)
    if (parent == "") {
        return current
    }

    return goUp(nodeMap, parent)
}

fun findParent(nodeMap: HashMap<String, Node>, current: String): String {
    nodeMap.forEach {
        if (it.value.children.contains(current)) {
            return it.key
        }
    }

    return ""
}

