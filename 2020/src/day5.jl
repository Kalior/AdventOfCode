module Day5

include("InputHelper.jl")

function get_input()::Array{String,1}
    pattern = r"(?<key>\S+):(?<value>\S+)"

    InputHelper.parse(line -> line, "5", "\n")
end

function solve()

    input = get_input()

    println("Part one $(solve_part_one(input))")

    println("Part two $(solve_part_two(input))")
end

function solve_part_one(boardingpasses::Array{String,1})
    maximum(map(get_pass_id, boardingpasses))
end

function get_pass_id(pass::String)::Int
    row = get_pass_row(pass)
    col = get_pass_col(pass)

    row * 8 + col
end

function get_pass_row(pass::String)
    upper_bound = 127
    lower_bound = 0
    for i in 1:7
        size = (upper_bound - lower_bound + 1)
        if pass[i] == 'B'
            lower_bound += size / 2
        elseif pass[i] == 'F'
            upper_bound -= size / 2
        end
    end

    upper_bound
end

function get_pass_col(pass::String)
    upper_bound = 7
    lower_bound = 0
    for i in 8:10
        size = (upper_bound - lower_bound + 1)
        if pass[i] == 'R'
            lower_bound += size / 2
        elseif pass[i] == 'L'
            upper_bound -= size / 2
        end
    end

    upper_bound
end

function solve_part_two(boardingpasses::Array{String,1})
    ids = map(get_pass_id, boardingpasses)
    id_set = Set(ids)

    for row in 0:127
        for col in 0:7
            id = row * 8 + col
            if in(id + 1, id_set) && in(id - 1, id_set) && !in(id, id_set)
                return id
            end
        end
    end

    nothing
end

end
