module Day7

include("InputHelper.jl")

function get_input()::Dict{String,Array{Tuple{Int,String}}}
    inputs = InputHelper.parse(line -> split(line, " bags contain "), "7", "\n")

    Dict(bag => parse_bags(rest) for (bag, rest) in inputs)
end

function parse_bags(rest)
    pattern = r"(?<count>[0-9]+) (?<color>[^,^.]+) bags?[,.]"

    [(parse(Int, m[:count]), m[:color]) for m in eachmatch(pattern, rest)]
end

function solve()
    input = get_input()

    println("Part one $(solve_part_one(input))")

    println("Part two $(solve_part_two(input))")
end

function solve_part_one(input::Dict{String,Array{Tuple{Int,String}}})
    find_bag = "shiny gold"
    sum(c -> bag_contains(c, input), filter(c -> c != find_bag, keys(input)))
end

function bag_contains(color::String, input::Dict{String,Array{Tuple{Int,String}}})
    find_bag = "shiny gold"

    if contains(color, find_bag)
        return true
    end

    any([bag_contains(bag_color, input) for (_, bag_color) in input[color]])
end


function n_bags_inside(color::String, input::Dict{String,Array{Tuple{Int,String}}})
    n = 0
    for (count, bag_color) in input[color]
        n += count

        n += count * n_bags_inside(bag_color, input)
    end

    n
end

function solve_part_two(input::Dict{String,Array{Tuple{Int,String}}})
    n_bags_inside("shiny gold", input)
end


end
