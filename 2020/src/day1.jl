function solve()
    input = get_input()

    println("Part one $(solve_part_one(input))")

    println("Part two $(solve_part_two(input))")
end

function get_input()::Array{Int,1}
    input = []

    input_path = joinpath(@__DIR__, "../inputs/day1")
    open(input_path, "r") do f
        for line in readlines(f)
            push!(input, parse(Int, line))
        end
    end

    input
end

function solve_part_one(input::Array{Int,1})
    for i in input
        for j in input
            if sums_to_2020(i, j)
                return i * j
            end
        end
    end

    nothing
end


function solve_part_two(input::Array{Int,1})
    for i in input
        for j in input
            for k in input
                if sums_to_2020(i, j, k)
                    return i * j * k
                end
            end
        end
    end
    -1
end

sums_to_2020(xs::Int...) = sum(xs) == 2020
