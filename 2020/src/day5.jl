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

function get_pass_row(pass::String)::Int
    str = map(c -> c ==  'F' ? '0' : '1', collect(pass[1:7]))
    parse(Int, String(str), base=2)
end

function get_pass_col(pass::String)::Int
    str = map(c -> c ==  'L' ? '0' : '1', collect(pass[8:10]))
    parse(Int, String(str), base=2)
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
