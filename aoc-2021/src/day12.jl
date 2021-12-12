module Day12
using Statistics
using Base.Iterators

include("ReadHelper.jl")

struct Point
    x::Int
    y::Int
end

function parse_input()::Dict{String,Array{String}}
    input = ReadHelper.getInputMap(line -> split(line, "-"), "12", "\n")

    connections = Dict()
    for (from, to) in input
        if !(from in keys(connections))
            connections[from] = []
        end
        if !(to in keys(connections))
            connections[to] = []
        end

        push!(connections[from], to)
        push!(connections[to], from)
    end

    connections
end

function solve()
    connections = parse_input()

    (solve_part_one(copy(connections)), solve_part_two(copy(connections)))
end

function solve_part_one(connections)
    working_paths = [["start"]]
    finished_paths = []

    while !isempty(working_paths)
        path = pop!(working_paths)

        connected_caves = connections[path[end]]

        for cave in connected_caves
            new_path = copy(path)
            push!(new_path, cave)

            if cave == "start" || filter_lower_case(cave, path)
                continue
            elseif cave == "end"
                push!(finished_paths, new_path)
            else
                push!(working_paths, new_path)
            end
        end
    end

    length(finished_paths)
end


function solve_part_two(connections)
    working_paths = [(["start"], false)]
    finished_paths = []

    while !isempty(working_paths)
        (path, has_visited_small_cave_twice) = pop!(working_paths)

        connected_caves = connections[path[end]]

        for cave in connected_caves
            new_path = copy(path)
            push!(new_path, cave)

            if cave == "start" || (filter_lower_case(cave, path) && has_visited_small_cave_twice)
                continue
            elseif cave == "end"
                push!(finished_paths, (new_path, has_visited_small_cave_twice))
            elseif filter_lower_case(cave, path) && !has_visited_small_cave_twice
                push!(working_paths, (new_path, true))
            else
                push!(working_paths, (new_path, has_visited_small_cave_twice))
            end
        end
    end

    length(finished_paths)
end

filter_lower_case(cave, path) = all(islowercase(c) for c in cave) && cave in path

end

Day12.solve()
