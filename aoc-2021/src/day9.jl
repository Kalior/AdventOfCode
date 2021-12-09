module Day9
include("ReadHelper.jl")

struct Point
    x::Int
    y::Int
end

function parse_input()
    input = ReadHelper.getInputMap(line -> [parse(Int, c) for c in line], "9", "\n")

    heightmap = Dict()
    for (i, line) in enumerate(input)
        for (j, h) in enumerate(line)
            heightmap[Point(i, j)] = h
        end
    end
    heightmap
end

function solve()
    heightmap = parse_input()

    (solve_part_one(heightmap), solve_part_two(heightmap))
end

function solve_part_one(heightmap)
    total_risk_level = 0
    for (point, height) in heightmap
        adjacent_points = get_adjacent(point)

        is_lowest = all(get(heightmap, adjacent_point, typemax(Int)) > height for adjacent_point in adjacent_points)
        if is_lowest
            total_risk_level += height + 1
        end
    end
    total_risk_level
end

function get_adjacent(p::Point)
    [Point(p.x - 1, p.y), Point(p.x + 1, p.y), Point(p.x, p.y - 1), Point(p.x, p.y + 1)]
end

function solve_part_two(heightmap)
    basin_sizes = []
    explored_points = Set()
    for (point, height) in heightmap
        if point in explored_points || height == 9
            continue
        end

        basin = Set([point])
        new_basin = Set(p for p in get_adjacent(point) if p in keys(heightmap) && heightmap[p] != 9)
        union!(new_basin, basin)

        while length(new_basin) != length(basin)
            basin = new_basin

            new_basin = Set(p for pp in basin for p in get_adjacent(pp) if p in keys(heightmap) && heightmap[p] != 9)
            union!(new_basin, basin)
        end
        union!(explored_points, basin)
        push!(basin_sizes, length(new_basin))
    end

    prod(sort(basin_sizes)[end-2:end])
end

end

Day9.solve()
