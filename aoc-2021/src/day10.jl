module Day10
using Statistics
include("ReadHelper.jl")

struct Point
    x::Int
    y::Int
end

function parse_input()
    ReadHelper.getInputMap(line -> line, "10", "\n")
end

function solve()
    input = parse_input()

    (solve_part_one(input), solve_part_two(input))
end

function solve_part_one(input)
    sum(score_corrupted_line(line) for line in input)
end

function score_corrupted_line(line)
    stack = []
    for c in line
        if c in ['(', '[', '{', '<']
            push!(stack, c)
        else
            opened_c = pop!(stack)
            if c == ')' && opened_c != '('
                return 3
            elseif c == ']' && opened_c != '['
                return 57
            elseif c == '}' && opened_c != '{'
                return 1197
            elseif c == '>' && opened_c != '<'
                return 25137
            end
        end
    end

    return 0
end

function score_incomplete_line(line)
    stack = []
    for c in line
        if c in ['(', '[', '{', '<']
            push!(stack, c)
        else
            pop!(stack)
        end
    end

    sum_ = 0
    while !isempty(stack)
        c = pop!(stack)
        sum_ = sum_ * 5
        if c == '('
            sum_ += 1
        elseif c == '['
            sum_ += 2
        elseif c == '{'
            sum_ += 3
        elseif c == '<'
            sum_ += 4
        end
    end

    return sum_
end


function solve_part_two(input)::Int
    non_corrupted_lines = [line for line in input if score_corrupted_line(line) == 0]

    scored_lines = [score_incomplete_line(line) for line in non_corrupted_lines]

    median(scored_lines)
end

end

Day10.solve()
