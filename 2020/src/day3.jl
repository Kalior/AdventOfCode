module Day3

include("InputHelper.jl")

function get_input()::Array{String,1}
    InputHelper.parse("3", line -> line)
end

function solve()

    input = get_input()

    println("Part one $(solve_part_one(input))")

    println("Part two $(solve_part_two(input))")
end

function solve_part_one(input::Array{String,1})
    y_tragectory = 1
    x_tragectory = 3

    count_trees(input, y_tragectory, x_tragectory)
end

function count_trees(input::Array{String,1}, y_tragectory::Int, x_tragectory::Int)::Int
    y_coord, x_coord = 1, 1
    n_trees = 0

    while y_coord <= length(input)
        if x_coord > length(input[1])
            x_coord -= length(input[1])
        end

        if input[y_coord][x_coord] == '#'
            n_trees += 1
        end

        y_coord += y_tragectory
        x_coord += x_tragectory

    end

    n_trees
end

function solve_part_two(input)
    trajectories = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
    prod(map(t -> count_trees(input, t[1], t[2]), trajectories))
end

end
