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
    play_game(input, 2020)
end


function solve_part_two(input)
    play_game(input, 30000000)
end


function play_game(numbers::Array{Int}, rounds::Int)
    spoken = Dict(number => turn for (turn, number) in enumerate(numbers))

    current_number = numbers[end]
    current_turn = length(numbers)

    while current_turn < rounds
        last_turn = get(spoken, current_number, current_turn)

        spoken[current_number] = current_turn

        current_number = current_turn - last_turn

        current_turn += 1
    end
    current_number
end

end
