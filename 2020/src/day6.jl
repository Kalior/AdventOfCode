module Day6

include("InputHelper.jl")

function get_input()::Array{String,1}
    InputHelper.parse(line -> line, "6", "\n\n")
end

function solve()
    input = get_input()

    println("Part one $(solve_part_one(input))")

    println("Part two $(solve_part_two(input))")
end

function solve_part_one(input::Array{String,1})
    sum(n_questions_answered, input)
end

function n_questions_answered(group::String)::Int
    length(Set(c for line in split(group, "\n") for c in line))
end


function solve_part_two(input::Array{String,1})
    sum(all_answered, input)
end

function all_answered(group::String)::Int
    length(intersect([[c for c in line] for line in split(group, "\n")]...))
end

end
