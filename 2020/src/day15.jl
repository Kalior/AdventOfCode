module Day15

include("InputHelper.jl")

function get_input()::Array{Int,1}
    InputHelper.parse(line -> parse(Int, line), "15", ",")
end

function solve()

    input = get_input()

    solve_part_one(input), solve_part_two(input)
end

function solve_part_one(input)
    spoken = Dict(number => (turn) for (turn, number) in enumerate(input))
    println(spoken)

    current_number = input[end]
    current_turn = length(input)

    while current_turn < 2020
        last_turn = get(spoken, current_number, current_turn)

        spoken[current_number] = current_turn
        current_number = current_turn - last_turn


        current_turn += 1
    end
    current_number
end



function solve_part_two(input)
    spoken = Dict(number => (turn) for (turn, number) in enumerate(input))
    println(spoken)

    current_number = input[end]
    current_turn = length(input)

    while current_turn < 30000000
        last_turn = get(spoken, current_number, current_turn)

        spoken[current_number] = current_turn
        current_number = current_turn - last_turn


        current_turn += 1
    end
    current_number
end

end
