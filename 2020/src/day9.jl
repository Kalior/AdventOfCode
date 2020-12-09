module Day9

include("InputHelper.jl")

function get_input()
    InputHelper.parse(line -> parse(Int, line), "9")
end

function solve()

    input = get_input()

    println("Part one $(solve_part_one(input))")

    println("Part two $(solve_part_two(input))")
end

function solve_part_one(input)
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


function solve_part_two(input)
    invalid_number = 1639024365

    for i in 1:length(input)
        rolling_sum = sum(input[i:i + 2])
        for j in 3:(length(input) - i)
            end_i = i + j
            rolling_sum += input[end_i]
            if rolling_sum == invalid_number
                min = minimum(input[i:end_i])
                max = maximum(input[i:end_i])
                println("min: $min, max: $max, block size: $j")
                return min + max
            elseif rolling_sum > invalid_number
                break
            end
        end
    end
    nothing
end

end
