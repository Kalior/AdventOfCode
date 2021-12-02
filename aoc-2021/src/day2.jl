using Base.Iterators
using Base

module Day2
include("ReadHelper.jl")

@enum Direction begin
    forward
    down
    up
end
dict = Dict("forward" => forward, "down" => down, "up" => up)

struct Instruction
    direction::Direction
    amount::Int
end

function parse_line(line)
    res = split(line, " ")
    amount = Base.parse(Int, res[2])

    Instruction(dict[res[1]], amount)
end


function parse()::Array{Instruction,1}
    ReadHelper.getInputMap(parse_line, "2", "\n")
end

function solve()
    input = parse()

    println(solve_part_one(input))
    println(solve_part_two(input))
end

function solve_part_one(input::Array{Instruction,1})
    forward_amount = 0
    depth_amount = 0
    for inst in input
        if inst.direction == forward
            forward_amount += inst.amount
        elseif inst.direction == down
            depth_amount += inst.amount
        elseif inst.direction == up
            depth_amount -= inst.amount
        end
    end

    forward_amount * depth_amount
end

function solve_part_two(input)
    forward_amount = 0
    depth_amount = 0
    aim_amount = 0

    for inst in input
        if inst.direction == forward
            forward_amount += inst.amount
            depth_amount += aim_amount * inst.amount
        elseif inst.direction == down
            aim_amount += inst.amount
        elseif inst.direction == up
            aim_amount -= inst.amount
        end
    end

    forward_amount * depth_amount
end

end
