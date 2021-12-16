module Day15
using Statistics
using Base.Iterators

include("ReadHelper.jl")

struct Point
    x::Int
    y::Int
end

function parse_input()
    input = ReadHelper.getInputMap(line -> [parse(Int, v) for v in line], "15", "\n")
    hcat(input...)
end

function solve()
    risk_levels = parse_input()
    #    @show risk_levels

    (solve_part_one(risk_levels), solve_part_two(risk_levels))
end


function solve_part_one(risk_levels)
    return astar(risk_levels)
end


function astar(risk_levels)
    risk_path = Dict([CartesianIndex(1, 1) => 0])
    (i_max, j_max) = size(risk_levels)

    nodes_to_reach = Set([CartesianIndex(1, 1)])
    guess_values = Dict(CartesianIndex(i, j) => typemax(Int) for i = 1:i_max for j = 1:j_max)

    end_i = CartesianIndex(i_max, j_max)

    while !isempty(nodes_to_reach)
        best_n = argmin(n -> guess_values[n], nodes_to_reach)

        if best_n == end_i
            return risk_path[end_i]
        end

        delete!(nodes_to_reach, best_n)

        reachable = [
            CartesianIndex(best_n[1], best_n[2] + 1),
            CartesianIndex(best_n[1], best_n[2] - 1),
            CartesianIndex(best_n[1] + 1, best_n[2]),
            CartesianIndex(best_n[1] - 1, best_n[2]),
        ]

        for adjacent in reachable
            if adjacent[1] < 1 || adjacent[1] > i_max || adjacent[2] < 1 || adjacent[2] > j_max
                continue
            end

            adjacent_score = risk_levels[adjacent] + risk_path[best_n]
            if get(risk_path, adjacent, typemax(Int)) > adjacent_score
                risk_path[adjacent] = adjacent_score
                guess_values[adjacent] = adjacent_score + (i_max - adjacent[1]) + (j_max - adjacent[2])
                push!(nodes_to_reach, adjacent)
            end
        end
    end


    @show risk_path[CartesianIndex(i_max, j_max)]
    @show risk_path[CartesianIndex(i_max, j_max - 1)]
    @show risk_path[CartesianIndex(i_max - 1, j_max)]
    risk_path[end_i]
end

function solve_part_two(risk_levels)
    (i_max, j_max) = size(risk_levels)
    extended_risk_levels = zeros(Int, size(risk_levels) .* 5)

    for i = 0:4
        for j = 0:4
            s = risk_levels
            for _ = 1:i+j
                s = (s .+ 1) .% 10
                s[findall(iszero, s)] .= 1
            end
            extended_risk_levels[1+i_max*i:i_max*(i+1), 1+j_max*j:j_max*(j+1)] = s
        end
    end

    return astar(extended_risk_levels)

end

end

Day15.solve()
