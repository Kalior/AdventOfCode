package day16

interface Move {
    fun act(programs: MutableList<Char>)
}

data class Spin(private val amount: Int) : Move {
    override fun act(programs: MutableList<Char>) {
        val length = programs.size

        programs.toList().forEachIndexed { i, c ->
            programs[(i + amount) % length] = c
        }
    }
}

data class Exchange(private val firstPosition: Int, private val secondPosition: Int) : Move {
    override fun act(programs: MutableList<Char>) {
        val firstProgram = programs[firstPosition]
        val secondProgram = programs[secondPosition]

        programs[secondPosition] = firstProgram
        programs[firstPosition] = secondProgram
    }
}

data class Partner(private val firstProgram: Char, private val secondProgram: Char) : Move {
    override fun act(programs: MutableList<Char>) {
        var firstPosition = -1
        var secondPosition = -1
        programs.forEachIndexed { i, c ->
            if (c == firstProgram) firstPosition = i
            if (c == secondProgram) secondPosition = i
        }

        programs[secondPosition] = firstProgram
        programs[firstPosition] = secondProgram
    }
}

fun parseSpin(move: String): Spin {
    val times = move.slice((1 until move.length)).toInt()
    return Spin(times)
}

fun parseExchange(move: String) : Exchange {
    val involved = move.slice((1 until move.length)).split("/")
    val firstPos = involved[0].toInt()
    val secondPos = involved[1].toInt()

    return Exchange(firstPos, secondPos)
}

fun parsePartner(move: String): Partner {
    val involved = move.slice((1 until move.length)).split("/")
    val firstProgram = involved[0].toCharArray()[0]
    val secondProgram = involved[1].toCharArray()[0]

    return Partner(firstProgram, secondProgram)
}