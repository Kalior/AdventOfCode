module Day2

include("InputHelper.jl")

function get_input()::Array{Tuple{Int,Int,Char,String},1}
    pattern = r"(?<min>\d+)-(?<max>\d+) (?<char>[a-z]): (?<passphrase>[a-z]+)"


    input = InputHelper.parse("2", line -> begin
        m = match(pattern, line)
        (parse(Int, m[:min]), parse(Int, m[:max]), first(m[:char]), m[:passphrase])
    end)

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

    min <= occurrences <= max
end


function valid_passphrase_two((first, second, char, passphrase)::Tuple{Int,Int,Char,String})
    return sum(i -> passphrase[i] == char, [first, second]) == 1
end

end
