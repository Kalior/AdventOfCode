module Day10

include("InputHelper.jl")

function get_input()::Array{Int}
    InputHelper.parse(line -> parse(Int, line), "10", "\n")
end

function solve()

    input = get_input()
    sort!(input)
    prepend!(input, 0)

    solve_part_one(input), solve_part_two(input)
end

function solve_part_one(input::Array{Int})::Int
    one_jolt_diff = 0
    three_jolt_diff = 0

    for i in 2:length(input)
        current_jolt = input[i - 1]

        if input[i] <= current_jolt + 3
            if input[i] == current_jolt + 1
                one_jolt_diff += 1
            elseif input[i] == current_jolt + 3
                three_jolt_diff += 1
            end
        end
    end

    three_jolt_diff += 1

    one_jolt_diff * three_jolt_diff
end


function solve_part_two(input::Array{Int})::Int
    ways_to_get_to_n = Array{Int}(undef, length(input))
    fill!(ways_to_get_to_n, 0)
    ways_to_get_to_n[1] = 1

    for i in 2:length(input)
        ways_to_get_to_i = 0
        for j in 0:min(3,  i - 1)
            if input[i - j] >= input[i] - 3
                ways_to_get_to_i += ways_to_get_to_n[i - j]
            end
        end
        ways_to_get_to_n[i] = ways_to_get_to_i
    end

    ways_to_get_to_n[end]
end


end
