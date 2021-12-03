using Base.Iterators
using Base

module Day3
include("ReadHelper.jl")


function parse()::Array{String,1}
    ReadHelper.getInputMap(f -> f, "3", "\n")
end

function solve()
    input = parse()

    println(solve_part_one(input))
    println(solve_part_two(input))
end

function solve_part_one(input::Array{String,1})
    pos_to_freq = Dict(i => 0 for (i, _) in enumerate(input[1]))
    for line in input
        for (i, c) in enumerate(line)
            if c == '1'
                pos_to_freq[i] += 1
            end
        end
    end

    most_common_bits = [most_common_bit(i, pos_to_freq, input) for (i, _) in enumerate(input[1])]
    least_common_bits = [least_common_bit(i, pos_to_freq, input) for (i, _) in enumerate(input[1])]

    gamma_rate = Base.parse(Int, string(most_common_bits...); base = 2)
    epsilon_rate = Base.parse(Int, string(least_common_bits...); base = 2)

    gamma_rate * epsilon_rate
end

function most_common_bit(index, dict, input)
    if dict[index] > length(input) - dict[index]
        return "1"
    else
        return "0"
    end
end
function least_common_bit(index, dict, input)
    if dict[index] > length(input) - dict[index]
        return "0"
    else
        return "1"
    end
end

function solve_part_two(input)
    oxygen_generator_rating = find_oxygen_generator_rating(input)
    c02_scrubber_rating = find_c02_scrubber_rating(input)

    oxygen_generator_rating * c02_scrubber_rating
end

function find_oxygen_generator_rating(input)
    bit_strings = copy(input)

    bit_string_length = lenght(bit_string[1])
    for i in range(bit_string_length)
        if length(bit_strings) == 1
            return Base.parse(Int, bit_strings[1]; base = 2)
        end

        bit_frequency = frequency_of_bits(i, bit_strings)

        most_common_bit = bit_frequency['0'] > bit_frequency['1'] ? '0' : '1'

        bit_strings = filter(bit_string -> bit_string[i] == most_common_bit, bit_strings)
    end

    if length(bit_strings) == 1
        return Base.parse(Int, bit_strings[1]; base = 2)
    else
        println("FAILED")
        return 0
    end
end

function find_c02_scrubber_rating(input)
    bit_strings = copy(input)

    bit_string_length = lenght(bit_string[1])
    for i in range(bit_string_length)
        if length(bit_strings) == 1
            return Base.parse(Int, bit_strings[1]; base = 2)
        end

        bit_frequency = frequency_of_bits(i, bit_strings)
        least_common_bit = bit_frequency['0'] <= bit_frequency['1'] ? '0' : '1'

        bit_strings = filter(bit_string -> bit_string[i] == least_common_bit, bit_strings)
    end

    if length(bit_strings) == 1
        return Base.parse(Int, bit_strings[1]; base = 2)
    else
        println("FAILED")
        return 0
    end
end

function frequency_of_bits(index, bit_strings)
    bit_frequency = Dict('0' => 0, '1' => 0)
    for bit_string in bit_strings
        c = bit_string[i]
        bit_frequency[c] += 1
    end
    return bit_frequency
end

end
