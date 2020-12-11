module Day11

include("InputHelper.jl")

function get_input()::Array{String,1}
    InputHelper.parse(line -> line, "11", "\n")
end

function solve()

    input = get_input()

    solve_part_one(input), solve_part_two(input)
end

function solve_part_one(input)
    new_seats = update_seats(input)
    old_seats = input

    while join(new_seats, "\n") != join(old_seats, "\n")
        old_seats = new_seats

        new_seats = update_seats(new_seats)
    end

    occupied_seats(new_seats)
end

function update_seats(input)::Array{String}
    new_seats = []

    for y in 1:length(input)
        row = Array{Char}(undef, 0)
        for x in 1:length(input[y])
            if input[y][x] == '.'
                push!(row, '.')
                continue
            end
            occupied = adjacent_occupied_seats(x, y, input)

            if input[y][x] == 'L'
                if occupied == 0
                    push!(row, '#')
                else
                    push!(row, 'L')
                end
            elseif input[y][x] == '#'
                if occupied >= 4
                    push!(row, 'L')
                else
                    push!(row, '#')
                end
            end
        end
        push!(new_seats, String(row))
    end

    new_seats
end

function adjacent_occupied_seats(x::Int, y::Int, input)
    n = 0
    test_coords = [(y - 1, x - 1), (y - 1, x), (y - 1, x + 1), (y, x - 1), (y, x + 1), (y + 1, x - 1), (y + 1, x), (y + 1, x + 1)]
    for (y_, x_) in test_coords
        if x_  > 0 && y_ > 0 &&  y_ <= length(input) && x_ <= length(input[y_])
            if input[y_][x_] == '#'
                n += 1
            end
        end
    end
    n
end

function occupied_seats(seats)
    sum(c == '#' for c in join(seats))
end


function solve_part_two(input)
    new_seats = update_seats_2(input)
    old_seats = input

    while join(new_seats, "\n") != join(old_seats, "\n")
        old_seats = new_seats

        new_seats = update_seats_2(new_seats)
    end

    occupied_seats(new_seats)
end

function update_seats_2(input)::Array{String}
    new_seats = []

    for y in 1:length(input)
        row = Array{Char}(undef, 0)
        for x in 1:length(input[y])
            if input[y][x] == '.'
                push!(row, '.')
                continue
            end
            occupied = visible_occupied_seats(x, y, input)

            if input[y][x] == 'L'
                if occupied == 0
                    push!(row, '#')
                else
                    push!(row, 'L')
                end
            elseif input[y][x] == '#'
                if occupied >= 5
                    push!(row, 'L')
                else
                    push!(row, '#')
                end
            end
        end
        push!(new_seats, String(row))
    end

    new_seats
end

function visible_occupied_seats(x::Int, y::Int, input)
    left = [(y, x - i) for i in 1:length(input[y])]
    right = [(y, x + i) for i in 1:length(input[y])]
    up = [(y + i, x) for i in 1:length(input)]
    down = [(y - i, x) for i in 1:length(input)]
    up_right_diagonal = [(y + i, x + i) for i in 1:min(length(input), length(input[y]))]
    up_left_diagonal = [(y + i, x - i) for i in 1:min(length(input), length(input[y]))]
    down_right_diagonal = [(y - i, x + i) for i in 1:min(length(input), length(input[y]))]
    down_left_diagonal = [(y - i, x - i) for i in 1:min(length(input), length(input[y]))]


    ranges = [left, right, up, down, up_right_diagonal, up_left_diagonal, down_left_diagonal, down_right_diagonal]
    seen_seats = filter(seat -> seat !== nothing, [get_first_visible_seat(range, input) for range in ranges])

    sum(input[y_][x_] == '#' for (y_, x_) in seen_seats)
end

function get_first_visible_seat(range, input)
    for (y_, x_) in range
        if y_ <= 0 || x_ <= 0 || y_ > length(input) || x_ > length(input[y_])
            break
        end
        if input[y_][x_] != '.'
            return (y_, x_)
        end
    end
    nothing
end

end
