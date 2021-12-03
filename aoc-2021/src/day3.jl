using Base.Iterators
using Base

module Day3
include("ReadHelper.jl")


function parse_input()::Array{String,1}
    ReadHelper.getInputMap(f -> f, "3", "\n")
end

function solve()
    input = parse_input()

    println(solve_part_one(input))
    println(solve_part_two(input))
end

function solve_part_one(input::Array{String,1})
    bit_string_length = length(input[1])
    pos_to_freq = Dict(i => 0 for i in range(1, stop = bit_string_length))

    for line in input
        for (i, c) in enumerate(line)
            if c == '1'
                pos_to_freq[i] += 1
            end
        end
    end

    most_common_bits = [most_common_bit(i, pos_to_freq, input) for i in range(1, stop = bit_string_length)]
    least_common_bits = [least_common_bit(i, pos_to_freq, input) for i in range(1, stop = bit_string_length)]

    gamma_rate = parse(Int, string(most_common_bits...); base = 2)
    epsilon_rate = parse(Int, string(least_common_bits...); base = 2)

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
    oxygen_generator_rating = filter_bit_strings(input, get_most_common)
    c02_scrubber_rating = filter_bit_strings(input, get_least_common)

    oxygen_generator_rating * c02_scrubber_rating
end

get_most_common(bit_frequency) = bit_frequency['0'] > bit_frequency['1'] ? '0' : '1'
get_least_common(bit_frequency) = bit_frequency['0'] <= bit_frequency['1'] ? '0' : '1'

function filter_bit_strings(input, f::Function)
    bit_strings = copy(input)

    bit_string_length = length(bit_strings[1])
    for i in range(1, stop = bit_string_length)
        if length(bit_strings) == 1
            return parse(Int, bit_strings[1]; base = 2)
        end

        bit_frequency = frequency_of_bits(i, bit_strings)

        most_common_bit = f(bit_frequency)

        bit_strings = filter(bit_string -> bit_string[i] == most_common_bit, bit_strings)
    end

    parse(Int, bit_strings[1]; base = 2)
end

function frequency_of_bits(index, bit_strings)
    bit_frequency = Dict('0' => 0, '1' => 0)
    for bit_string in bit_strings
        c = bit_string[index]
        bit_frequency[c] += 1
    end
    return bit_frequency
end

end
