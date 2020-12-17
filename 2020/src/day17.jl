module Day17

include("InputHelper.jl")

function get_input()::Array{String,1}
    InputHelper.parse(line -> line, "17", "\n")
end

function solve()
    input = get_input()

    solve_part_one(input), solve_part_two(input)
end

function solve_part_one(input)
    solve_part(input, 3)
end

function solve_part_two(input)
    solve_part(input, 4)
end

function solve_part(input, n_dims::Int)
    map = initial_map(input)

    for _ in 1:6
        map = run_step(map, n_dims)
    end

    length(filter(k -> map[k] == :active, keys(map)))
end


function initial_map(input)
    map = Dict()
    for (y, line) in enumerate(input)
        for (x, c) in enumerate(line)
            if c == '#'
                map[(x, y, 0, 0)] = :active
            else
                map[(x, y, 0, 0)] = :inactive
            end
        end
    end
    map
end

function run_step(map, n_dims::Int)
    map_copy = copy(map)

    keys_and_neighbours = Set(c for coord in keys(map) for c in neighbour_states(coord, n_dims))

    for coordinate in keys_and_neighbours
        active_neighbours = active_neighbour_states(coordinate, map, n_dims)
        state = get(map, coordinate, :inactive)

        if state == :inactive && active_neighbours == 3
            map_copy[coordinate] = :active
        elseif state == :active && !in(active_neighbours, 2:3)
            map_copy[coordinate] = :inactive
        end
    end

    map_copy
end

function active_neighbour_states((x, y, z, w), map, n_dims)
    sum(get(map, n, :inactive) == :active for n in neighbour_states((x, y, z, w), n_dims))
end

function neighbour_states((x, y, z, w), n_dims)
    xs = [x_ for x_ in [x, x - 1, x + 1]]
    ys = [y_ for y_ in [y, y - 1, y + 1]]
    zs = [z_ for z_ in [z, z - 1, z + 1]]
    ws = [w_ for w_ in [w, w - 1, w + 1]]

    if n_dims == 3
        [(x_, y_, z_, 0) for x_ in xs for y_ in ys for z_ in zs if !(x_ == x && y_ == y && z_ == z)]
    elseif n_dims == 4
        [(x_, y_, z_, w_) for x_ in xs for y_ in ys for z_ in zs for w_ in ws if !(x_ == x && y_ == y && z_ == z && w_ == w)]
    end
end

end
