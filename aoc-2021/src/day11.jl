module Day11
using Statistics
using Base.Iterators

include("ReadHelper.jl")

struct Point
    x::Int
    y::Int
end

function parse_input()::Matrix{Int}
    input = ReadHelper.getInputMap(line -> [parse(Int, v) for v in line], "11", "\n")
    hcat(input...)
end

function solve()
    octupi = parse_input()

    (solve_part_one(copy(octupi)), solve_part_two(copy(octupi)))
end

function solve_part_one(octupi)
    sum(simulate_step!(octupi) for _ in range(1, 100))
end

function simulate_step!(octupi)
    octupi .+= 1
    has_flashed = falses(size(octupi))
    n_flashes = 0

    is_stable = false

    while !is_stable
        is_stable = true
        for i in CartesianIndices(octupi)

            if octupi[i] > 9 && !has_flashed[i]
                n_flashes += 1
                is_stable = false
                has_flashed[i] = true
                adjacent = all_adjacent(i)
                octupi[adjacent] .+= 1
            end
        end
    end

    octupi[has_flashed] .= 0

    n_flashes
end

all_adjacent(i) = [CartesianIndex(i_, j_) for (i_, j_) in product(i[1]-1:i[1]+1, i[2]-1:i[2]+1) if (1 <= i_ <= 10) && (1 <= j_ <= 10) && CartesianIndex(i_, j_) != i]



function solve_part_two(octupi)
    n_octupi = prod(size(octupi))

    findfirst(simulate_step!(octupi) == n_octupi for _ in range(1, 2000))
end

end

Day11.solve()
