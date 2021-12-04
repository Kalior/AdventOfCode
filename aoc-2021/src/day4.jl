using Base.Iterators
using Base

module Day4
include("ReadHelper.jl")



function parse_input()
    input::Array{String,1} = collect(ReadHelper.getInputMap(f -> f, "4", "\n\n"))
    numbers = map(n -> parse(Int, n), split(input[1], ","))
    boards = map(parse_board, input[2:end])

    (numbers, boards)
end

function parse_board(string::String)::Vector{Vector{Int}}
    rows = split(strip(string), "\n")
    [[parse(Int, n) for n in split(row, " ") if n != ""] for row in rows]
end

function solve()
    numbers, boards = parse_input()

    (solve_part_one(boards, numbers), solve_part_two(boards, numbers))
end

function solve_part_one(boards::Array{Vector{Vector{Int}},1}, numbers::Array{Int,1})
    results = map(board -> numbers_until_bingo(board, numbers), boards)
    minimum(results)
end

function numbers_until_bingo(board::Vector{Vector{Int}}, numbers::Array{Int,1})::Tuple{Int,Int}
    board_size = length(board)
    number_to_row = Dict(n => row_i for (row_i, row) in enumerate(board) for (col_i, n) in enumerate(row))
    number_to_col = Dict(n => col_i for (row_i, row) in enumerate(board) for (col_i, n) in enumerate(row))

    winning_numbers_per_row = Dict(i => 0 for i in range(1, board_size))
    winning_numbers_per_col = Dict(i => 0 for i in range(1, board_size))

    marked_positions::Set{Tuple{Int,Int}} = Set()

    for (i, num) in enumerate(numbers)
        if haskey(number_to_col, num)
            row_i = number_to_row[num]
            col_i = number_to_col[num]

            push!(marked_positions, (row_i, col_i))

            winning_numbers_per_col[col_i] += 1
            winning_numbers_per_row[row_i] += 1

            if winning_numbers_per_col[col_i] == board_size || winning_numbers_per_row[row_i] == board_size
                return (i, sum_of_unmarked_numbers(board, marked_positions) * num)
            end
        end
    end

    (typemax(Int), 0)
end

function sum_of_unmarked_numbers(board::Vector{Vector{Int}}, marked_positions::Set{Tuple{Int,Int}})
    sum = 0
    for (row_i, row) in enumerate(board)
        for (col_i, v) in enumerate(row)
            if (row_i, col_i) ∉ marked_positions
                sum += v
            end
        end
    end
    sum
end

function solve_part_two(boards::Array{Vector{Vector{Int}},1}, numbers::Array{Int,1})
    results = map(board -> numbers_until_bingo(board, numbers), boards)
    maximum(results)
end

end
