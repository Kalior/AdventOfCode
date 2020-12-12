module Day12

include("InputHelper.jl")

function get_input()::Array{String,1}
    InputHelper.parse(line -> line, "12", "\n")
end

function solve()::Tuple{Int,Int}

    input = get_input()

    solve_part_one(input), solve_part_two(input)
end

north(v, (x, y)) = (x, y + v)
south(v, (x, y)) = (x, y - v)
east(v, (x, y)) = (x + v, y)
west(v, (x, y)) = (x - v, y)
forward(v, pos, direction) = pos .+ v .* direction

function rotate(rotation, (w_x, w_y))::Tuple{Int,Int}
    rotation_matrix = [cosd(-rotation) -sind(-rotation); sind(-rotation) cosd(-rotation)]

    rotated = rotation_matrix * vec([w_x w_y])

    (rotated[1], rotated[2])
end


function solve_part_one(input)
    position = (0, 0)
    direction = (1, 0)
    for line in input
        action = line[1]
        value = parse(Int, line[2:end])
        if action == 'N'
            position = north(value, position)
        elseif action == 'E'
            position = east(value, position)
        elseif action == 'S'
            position = south(value, position)
        elseif action == 'W'
            position = west(value, position)
        elseif action == 'R'
            direction = rotate(value, direction)
        elseif action == 'L'
            direction = rotate(-value, direction)
        elseif action == 'F'
            position = forward(value, position, direction)
        end
    end

    abs(position[1]) + abs(position[2])
end


function solve_part_two(input)
    position = (0, 0)
    relative_waypoint_position = (10, 1)
    for line in input
        action = line[1]
        value = parse(Int, line[2:end])
        if action == 'N'
            relative_waypoint_position = north(value, relative_waypoint_position)
        elseif action == 'E'
            relative_waypoint_position = east(value, relative_waypoint_position)
        elseif action == 'S'
            relative_waypoint_position = south(value, relative_waypoint_position)
        elseif action == 'W'
            relative_waypoint_position = west(value, relative_waypoint_position)
        elseif action == 'R'
            relative_waypoint_position = rotate(value, relative_waypoint_position)
        elseif action == 'L'
            relative_waypoint_position = rotate(-value, relative_waypoint_position)
        elseif action == 'F'
            position = forward(value, position, relative_waypoint_position)
        end
    end

    abs(position[1]) + abs(position[2])
end


end
