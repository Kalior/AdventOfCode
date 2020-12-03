module Day1

include("InputHelper.jl")

function solve()
    input = get_input()

    println("Part one $(solve_part_one(input))")

    println("Part two $(solve_part_two(input))")
end

function get_input()::Array{Int,1}
    InputHelper.parse("1", line -> parse(Int, line))
end

function solve_part_one(input::Array{Int,1})
    for (i, j) in Iterators.product(input, input)
        if sums_to_2020(i, j)
            return i * j
        end
    end

    nothing
end


function solve_part_two(input::Array{Int,1})
    for (i, j, k) in Iterators.product(input, input, input)
        if sums_to_2020(i, j, k)
            return i * j * k
        end
    end

    nothing
end

sums_to_2020(xs::Int...) = sum(xs) == 2020
end
