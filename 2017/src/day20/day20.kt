package day20

import kotlin.system.measureTimeMillis

data class Input(val raw: List<String>, val particles: List<Particle>)

data class Particle(val position: Vector3, private val velocity: Vector3, private val acceleration: Vector3) {
    fun update(): Particle {
        val newVelocity = velocity + acceleration
        val newPosition = position + newVelocity
        return Particle(newPosition, newVelocity, acceleration)
    }

    fun distanceToZero(): Double {
        return Math.abs(position.x) + Math.abs(position.y) + Math.abs(position.z)
    }
}

data class Vector3(val x: Double, val y: Double, val z: Double) {
    operator fun plus(other: Vector3): Vector3 {
        val newX = this.x + other.x
        val newY = this.y + other.y
        val newZ = this.z + other.z
        return Vector3(newX, newY, newZ)
    }
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
    val parser = util.Parser("20")
    val lines = parser.getLines()
    val numbers = parser.getNumbersFromLines(lines)
    val particles = numbers.map { parseParticle(it) }

    return Input(lines, particles)
}

fun parseParticle(line: List<Double>): Particle {
    val position = Vector3(line[0], line[1], line[2])
    val velocity = Vector3(line[3], line[4], line[5])
    val acceleration = Vector3(line[6], line[7], line[8])
    return Particle(position, velocity, acceleration)
}

fun solve2(input: Input): String {
    var particles = input.particles
    var lastSize = 1001
    while (lastSize > particles.size) {
        lastSize = particles.size
        repeat(100) {
            particles = particles.map{ it.update() }
            particles = removeCollisions(particles)
        }
    }

    return "${particles.size}"
}

fun removeCollisions(particles: List<Particle>): List<Particle> {
    val map = mutableMapOf<Vector3, MutableList<Particle>>().withDefault { mutableListOf() }

    particles.forEach {
        val list = map.getValue(it.position)
        list.add(it)
        map.put(it.position, list)
    }

    val nonCollidingParticles = mutableListOf<Particle>()
    map.forEach { _, mutableList ->
        if (mutableList.size == 1) {
            nonCollidingParticles.add(mutableList[0])
        }
    }

    return nonCollidingParticles.toList()
}

fun solve1(input: Input): String {
    var particles = input.particles

    var lastClosestParticle = 0
    var closestParticle = 1

    while (lastClosestParticle != closestParticle) {
        repeat(200) {
            particles = particles.map{ it.update() }
        }
        lastClosestParticle = closestParticle
        closestParticle = closestToZero(particles)
    }

    return closestParticle.toString()
}

fun closestToZero(particles: List<Particle>): Int {
    return particles.indices.minBy { particles[it].distanceToZero() }!!
}
