using Base
module ReadHelper

function getInput(day::String)::Array{Int,1}
    path = joinpath(@__DIR__, "../inputs/day$day")
    lines = open(f -> read(f, String), path)

    map(line -> parse(Int, line), split(strip(lines), "\n"))
end


function getInputMap(f::Function, day::String, split_by::String = "\n")::Array{Any,1}
    input_path = joinpath(@__DIR__, "../inputs/day$day")
    input = []

    open(input_path, "r") do io
        input = read(io, String)
    end

    map(f, split(strip(input), split_by))
end

end
