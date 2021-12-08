

module Day8
using Statistics
include("ReadHelper.jl")


function parse_input()
    ReadHelper.getInputMap(line -> [split(part, " ") for part in split(line, " | ")], "8", "\n")
end

function solve()
    input = parse_input()

    (solve_part_one(input), solve_part_two(input))
end

function solve_part_one(input)
    sum(length(part) in [2, 3, 4, 7] for entry in input for part in entry[2])
end


function solve_part_two(input)
    sum(decode(entry) for entry in input)
end

function decode(entry)
    input = entry[1]
    output = entry[2]

    num_to_code = Dict(i => "" for i = 0:8)

    for num in input
        if length(num) == 2
            num_to_code[1] = num
        elseif length(num) == 3
            num_to_code[7] = num
        elseif length(num) == 4
            num_to_code[4] = num
        elseif length(num) == 7
            num_to_code[8] = num
        end
    end

    for num in input
        if !(length(num) in [2, 3, 4, 7])
            if both_ones(num, num_to_code[1]) && length(num) == 6
                ## 9 or 0
                if missing_any_from_four(num, num_to_code[4])
                    num_to_code[0] = num
                else
                    num_to_code[9] = num
                end
            elseif both_ones(num, num_to_code[1]) && length(num) == 5
                num_to_code[3] = num
            elseif n_missing_from_four(num, num_to_code[4]) == 2 && length(num) == 5
                num_to_code[2] = num
            elseif length(num) == 5
                num_to_code[5] = num
            else
                num_to_code[6] = num
            end
        end
    end

    reverse_code = Dict(join(sort(collect(v))) => k for (k, v) in num_to_code)

    output_vals = [string(reverse_code[join(sort(collect(num)))]) for num in output]

    parse(Int, join(output_vals))
end

both_ones(num, one_num) = all(contains(num, c) for c in one_num)
missing_any_from_four(num, four_num) = any(!contains(num, c) for c in four_num)
n_missing_from_four(num, four_num) = length([1 for c in four_num if !contains(num, c)])

end
