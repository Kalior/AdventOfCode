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

        all_continuations = connections[path[end]]

        for continuation in all_continuations
            new_path = copy(path)
            push!(new_path, continuation)

            if continuation == "start"
                continue
            elseif continuation == "end"
                push!(finished_paths, new_path)
            elseif all(islowercase(c) for c in continuation) && continuation in path
                continue
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

        continuations = connections[path[end]]

        for continuation in continuations
            new_path = copy(path)
            push!(new_path, continuation)

            if continuation == "start"
                continue
            elseif continuation == "end"
                push!(finished_paths, (new_path, has_visited_small_cave_twice))
            elseif all(islowercase(c) for c in continuation) && continuation in path
                if has_visited_small_cave_twice
                    continue
                else
                    push!(working_paths, (new_path, true))
                end
            else
                push!(working_paths, (new_path, has_visited_small_cave_twice))
            end
        end
    end

    length(finished_paths)
end

end

Day12.solve()
