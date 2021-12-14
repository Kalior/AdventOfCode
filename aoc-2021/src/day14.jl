module Day14
using Statistics
using Base.Iterators

include("ReadHelper.jl")

struct Point
    x::Int
    y::Int
end

function parse_input()
    input = ReadHelper.getInputMap(line -> line, "14", "\n\n")

    polymer = input[1]
    instructions = input[2]

    polymer, Dict([split(instr, " -> ") for instr in split(instructions, "\n")])
end

function solve()
    polymer, instructions = parse_input()

    (solve_part_one(polymer, instructions), solve_part_two(polymer, instructions))
end

function solve_part_one(polymer, instructions)
    for _ = 1:10
        new_polymer = []
        for i = 1:length(polymer)-1
            pair = polymer[i:i+1]
            new_element = instructions[pair]
            push!(new_polymer, polymer[i])
            push!(new_polymer, new_element)
        end
        push!(new_polymer, polymer[end])
        polymer = join(new_polymer)
    end

    counts = Dict()
    for e in polymer
        c = get(counts, e, 0)
        counts[e] = c + 1
    end

    maximum(values(counts)) - minimum(values(counts))
end


function solve_part_two(polymer, instructions)::Int64
    pairs = Dict(polymer[i:i+1] => 0 for i = 1:length(polymer)-1)
    for i = 1:length(polymer)-1
        pair = polymer[i:i+1]
        c = get(pairs, pair, 0)
        pairs[pair] = c + 1
    end

    for _ = 1:40
        new_pairs = Dict()
        for (pair, c) in pairs
            new_element = instructions[pair]

            left_pair = join([pair[1], new_element])
            left_c = get(new_pairs, left_pair, 0)
            new_pairs[left_pair] = left_c + c


            right_pair = join([new_element, pair[2]])
            right_c = get(new_pairs, right_pair, 0)
            new_pairs[right_pair] = right_c + c
        end
        pairs = new_pairs
    end

    counts = Dict()
    for (pair, c) in pairs
        e1 = pair[1]
        c1 = get(counts, e1, 0)
        counts[e1] = c1 + c
    end

    # Final element is always the same and won't get counted above
    counts[polymer[end]] += 1

    maximum(values(counts)) - minimum(values(counts))
end

end

Day14.solve()
