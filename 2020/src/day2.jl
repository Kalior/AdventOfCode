module Day2

function get_input()::Array{Tuple{Int,Int,Char,String},1}
    input = []

    input_path = joinpath(@__DIR__, "../inputs/day2")

    pattern = r"(?<min>\d+)-(?<max>\d+) (?<char>[a-z]): (?<passphrase>[a-z]+)"

    open(input_path, "r") do f
        for line in readlines(f)
            m = match(pattern, line)
            min = parse(Int, m[:min])
            max = parse(Int, m[:max])
            push!(input, (min, max, first(m[:char]), m[:passphrase]))
        end
    end

    input
end

function solve()
    input = get_input()
    println("Part one $(solve_part_one(input))")

    println("Part two $(solve_part_two(input))")
end

function solve_part_one(input)
    count(valid_passphrase, input)
end

function solve_part_two(input)
    count(valid_passphrase_two, input)
end

function valid_passphrase((min, max, char, passphrase)::Tuple{Int,Int,Char,String})
    occurrences = count(c -> (c == char), passphrase)

    occurrences >= min && occurrences <= max
end


function valid_passphrase_two((first, second, char, passphrase)::Tuple{Int,Int,Char,String})
    occurrences = 0
    if passphrase[first] == char
        occurrences += 1
    end
    if passphrase[second] == char
        occurrences += 1
    end

    return occurrences == 1
end

end
