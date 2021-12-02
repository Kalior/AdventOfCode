using Base.Iterators

module Day1
include("ReadHelper.jl")


function parse()
    ReadHelper.getInput("1")
end

function solve()
    input = parse()

    println(solve_part_one(input))
    println(solve_part_two(input))
end

function solve_part_one(input)
    n_increases = 0
    for (first, second) in zip(input, Iterators.drop(input, 1))
        println(first)
        println(second)
        if second > first
            n_increases += 1
        end
    end

    n_increases
end

function solve_part_two(input)
    sliding_window = [a + b + c for (a, b, c) in zip(input, Iterators.drop(input, 1), Iterators.drop(input, 2))]

    n_increases = 0
    for (first, second) in zip(sliding_window, Iterators.drop(sliding_window, 1))
        if second > first
            n_increases += 1
        end
    end

    n_increases
end

end
