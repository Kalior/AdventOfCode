module Day0

include("InputHelper.jl")

function get_input()::Array{String,1}
    InputHelper.parse(line -> line, "0", "\n")
end

function solve()

    input = get_input()

    solve_part_one(input), solve_part_two(input)
end

function solve_part_one(input)
    nothing
end


function solve_part_two(input)
    nothing
end

end
