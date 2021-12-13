module Day13
using Statistics
using Base.Iterators

include("ReadHelper.jl")

struct Point
    x::Int
    y::Int
end

function parse_line(line)
    splits = split(line, ",")

    Point(parse(Int, splits[1]), parse(Int, splits[2]))
end

function parse_input()
    input = ReadHelper.getInputMap(line -> line, "13", "\n\n")

    dots = input[1]
    folds = input[2]


    parsed_dots = [parse_line(line) for line in split(dots, "\n")]
    parsed_folds = [split(split(line, " ")[end], "=") for line in split(folds, "\n")]

    parsed_dots, parsed_folds
end

function solve()
    parsed_dots, folds = parse_input()

    (solve_part_one(parsed_dots, folds), solve_part_two(parsed_dots, folds))
end

function solve_part_one(parsed_dots, folds)
    dots = Set(parsed_dots)

    folded_dots = fold(dots, folds[1])

    length(folded_dots)
end

function fold(dots, fold)

    fold_pos = parse(Int, fold[2])
    if fold[1] == "x"
        return Set(x_fold_dot(dot, fold_pos) for dot in dots)
    else
        return Set(y_fold_dot(dot, fold_pos) for dot in dots)
    end
end

function x_fold_dot(dot::Point, fold_pos)
    if dot.x < fold_pos
        return dot
    else
        new_x = fold_pos - (dot.x - fold_pos)
        return Point(new_x, dot.y)
    end
end

function y_fold_dot(dot::Point, fold_pos)
    if dot.y < fold_pos
        return dot
    else
        new_y = fold_pos - (dot.y - fold_pos)
        return Point(dot.x, new_y)
    end
end


function solve_part_two(parsed_dots, folds)
    dots = Set(parsed_dots)

    for fold_ in folds
        dots = fold(dots, fold_)
    end

    for i = 0:5
        for j = 0:38
            p = Point(j, i)
            if p in dots
                print("#")
            else
                print(" ")
            end
        end
        println()
    end
end

end

Day13.solve()
