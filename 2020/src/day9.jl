module Day9

include("InputHelper.jl")

function get_input()::Array{Int}
    InputHelper.parse(line -> parse(Int, line), "9")
end

function solve()
    input = get_input()

    println("Part one $(solve_part_one(input))")

    println("Part two $(solve_part_two(input))")
end

function solve_part_one(input::Array{Int})::Int
    preamble = 25
    for i in (preamble + 1):length(input)
        n = input[i]

        first_allowed = i - preamble

        is_valid = any(input[j] + input[k] == n for j in first_allowed:(i - 1) for k in first_allowed:(i - 1) if j != k)
        if !is_valid
            return n
        end
    end
    nothing
end


function solve_part_two(input::Array{Int})::Int
    invalid_number = 1639024365

    for start in 1:length(input)
        rolling_sum = sum(input[start:start + 2])
        for end_ in (start + 3):length(input)
            rolling_sum += input[end_]
            if rolling_sum == invalid_number
                min = minimum(input[start:end_])
                max = maximum(input[start:end_])
                println("min: $min, max: $max, block size: $(end_ - start)")
                return min + max
            elseif rolling_sum > invalid_number
                break
            end
        end
    end
    nothing
end

end
