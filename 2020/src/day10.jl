module Day10

include("InputHelper.jl")

function get_input()::Array{Int,1}
    InputHelper.parse(line -> parse(Int, line), "10", "\n")
end

function solve()

    input = get_input()

    println("Part one $(solve_part_one(input))")

    println("Part two $(solve_part_two(input))")
end

function solve_part_one(input)
    sort!(input)

    one_jolt_diff = 0
    three_jolt_diff = 0

    current_jolt = 0
    for i in 1:(length(input))
        if input[i] <= current_jolt + 3
            if input[i] == current_jolt + 1
                one_jolt_diff += 1
            elseif input[i] == current_jolt + 3
                three_jolt_diff += 1
            end
            current_jolt = input[i]
        end
    end

    three_jolt_diff += 1

    one_jolt_diff * three_jolt_diff
end


function solve_part_two(input)
    push!(input, 0)
    sort!(input)

    push!(input, input[end] + 3)

    ways_to_get_to_n = Array{Int}(undef, length(input))
    fill!(ways_to_get_to_n, 0)
    ways_to_get_to_n[1] = 1

    for i in 2:(length(input))
        ways_to_get_to_i = 0
        for j in 0:min(3,  i - 1)
            if input[i - j] >= input[i] - 3
                ways_to_get_to_i += ways_to_get_to_n[i - j]
            end
        end
        ways_to_get_to_n[i] = ways_to_get_to_i
    end

    previous_ns[end]
end


end
