

module Day7
using Statistics
include("ReadHelper.jl")


function parse_input()::Array{Int,1}
    ReadHelper.getInputMap(f -> parse(Int, f), "7", ",")
end

function solve()
    input = parse_input()

    (solve_part_one(input), solve_part_two(input))
end

function solve_part_one(input)
    median_ = Statistics.median(input)

    sum([abs(v - median_) for v in input])
end


function solve_part_two(input)::Tuple{Int,Int}
    mean_ = (mean(input))
    minimum((pos_cost(input, pos), pos) for pos in [floor(mean_), ceil(mean_)])
end

pos_cost(input, pos) = sum([sum(i for i in range(1, stop = abs(v - pos))) for v in input]) # Can look up the expression for the internal sum, but runs fast enough

end
