module Day8

include("InputHelper.jl")

function get_input()::Array{Tuple{Symbol,Int},1}
    input = InputHelper.parse(line -> split(line, " "), "8", "\n")

    [(Symbol(op), parse(Int, v)) for (op, v) in input]
end

function solve()

    input = get_input()

    println("Part one $(solve_part_one(input))")

    println("Part two $(solve_part_two(input))")
end

function solve_part_one(input)
    (val, _) = execute_until_infinite(input)
    val
end


function solve_part_two(input)
    for i in 1:length(input)
        (op, v) = input[i]
        if op == :acc
            continue
        end

        new_op = op == :jmp ? :nop : :jmp
        input[i] = (new_op, v)

        (acc, terminated) = execute_until_infinite(input)

        if terminated
            return acc
        end

        input[i] = (op, v)
    end

    nothing
end


acc(v, acc) = (v + acc, 1)
jmp(v, acc) = (acc, v)
nop(_, acc) = (acc, 1)

function execute_until_infinite(input)
    visited = Set()
    ip = 1
    accumulator = 0

    while ip <= length(input)
        if in(ip, visited)
            return accumulator, false
        end
        push!(visited, ip)
        (op, v) = input[ip]

        op_f = getfield(Day8, op)
        (accumulator, ip_offset) = op_f(v, accumulator)

        ip += ip_offset
    end

    accumulator, true
end

end
