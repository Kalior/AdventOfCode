module Day14

include("InputHelper.jl")

function get_input()::Array{String,1}
    InputHelper.parse(line -> line, "14", "\n")
end

function solve()

    input = get_input()

    solve_part_one(input), solve_part_two(input)
end

function solve_part_one(input)
    # mem[23173] = 12045
    pattern = r"mem\[(?<adress>\d+)\] = (?<value>\d+)"


    memory = Array{Int,1}(undef, 10000000)
    fill!(memory, 0)

    mask = []
    current_mask = 0
    for line in input
        if line[1:4] == "mask"
            mask = [to_mask(c) for c in split(line, " = ")[2]]
        else
            m = match(pattern, line)
            adress = parse(Int, m[:adress]) + 1
            value = parse(Int, m[:value])
            value_masked = mask_value(mask, value)

            memory[adress] = value_masked
        end
    end

    sum(memory)
end

function to_mask(c)
    if c == 'X'
        return nothing
    else
        return parse(Int, c)
    end
end


function solve_part_two(input)
    # mem[23173] = 12045
    pattern = r"mem\[(?<adress>\d+)\] = (?<value>\d+)"


    memory = Dict()

    mask = []
    current_mask = 0
    for line in input
        if line[1:4] == "mask"
            mask = [to_mask(c) for c in split(line, " = ")[2]]
        else
            m = match(pattern, line)
            adress = parse(Int, m[:adress])
            value = parse(Int, m[:value])
            adresses = mask_all_adresses(mask, adress)
            for masked_adress in adresses
                memory[masked_adress + 1] = value
            end
        end
    end

    sum(values(memory))
end

function mask_value(mask, value)
    str_rep = [parse(Int, b) for b in bitstring(value)]
    index_offset = 64 - 36
    for (i, bit) in enumerate(mask)
        if bit !== nothing
            str_rep[i + index_offset] = bit
        end
    end
    value_masked = parse(Int, join(str_rep), base=2)
    value_masked
end

function mask_all_adresses(mask, adress)
    str_rep = [parse(Int, b) for b in bitstring(adress)]

    index_offset = 64 - 36
    adresses = [str_rep]
    for (i, bit) in enumerate(mask)
        if bit === nothing
            for adr in adresses
                adr[i + index_offset] = 0
            end

            one_adreeses = []
            for adr in adresses
                adr_1 = copy(adr)
                adr_1[i + index_offset] = 1
                push!(one_adreeses, adr_1)
            end
            append!(adresses, one_adreeses)
        elseif bit == 1
            for adr in adresses
                adr[i + index_offset] = bit
            end
        end
    end


    [parse(Int, join(adr), base=2) for adr in adresses]
end

function mask_adress(mask, value)
    str_rep = [parse(Int, b) for b in bitstring(value)]
    index_offset = 64 - 36
    for (i, bit) in enumerate(mask)
        if bit == 1
            str_rep[i + index_offset] = bit
        end
    end
    value_masked = parse(Int, join(str_rep), base=2)
    value_masked
end

end
