using Base.Iterators

module Day5
include("ReadHelper.jl")


struct Point
    x::Int
    y::Int
end

function parse_points(line::SubString)
    (start_, end_) = split(line, " -> ")
    start_point = [parse(Int, v) for v in split(start_, ",")]
    end_point = [parse(Int, v) for v in split(end_, ",")]


    Point(start_point[1], start_point[2]), Point(end_point[1], end_point[2])
end

function parse_input()::Array{Tuple{Point,Point}}
    ReadHelper.getInputMap(parse_points, "5", "\n")
end

function solve()
    input = parse_input()

    (solve_part_one(input), solve_part_two(input))
end

function solve_part_one(input)
    map_of_airvents::Dict{Point,Int} = Dict()

    for (start_point, end_point) in input
        if is_horisontal(start_point, end_point)
            line_points = get_line_points(start_point, end_point)
            for point in line_points
                if haskey(map_of_airvents, point,)
                    map_of_airvents[point] += 1
                else
                    map_of_airvents[point] = 1
                end
            end
        end
    end

    sum(1 for v in values(map_of_airvents) if v > 1)
end

is_horisontal(start_point, end_point) = start_point.x == end_point.x || start_point.y == end_point.y

function get_line_points(start_point, end_point)
    dx = sign(end_point.x - start_point.x)
    dy = sign(end_point.y - start_point.y)
    point = start_point
    points = []
    while !(point.x == end_point.x && point.y == end_point.y)
        push!(points, point)
        point = Point(point.x + dx, point.y + dy)
    end
    push!(points, point)

    points
end

function solve_part_two(input)
    map_of_airvents::Dict{Point,Int} = Dict()

    for (start_point, end_point) in input
        line_points = get_line_points(start_point, end_point)
        for point in line_points
            if haskey(map_of_airvents, point,)
                map_of_airvents[point] += 1
            else
                map_of_airvents[point] = 1
            end
        end

    end

    sum(1 for v in values(map_of_airvents) if v > 1)
end

end
