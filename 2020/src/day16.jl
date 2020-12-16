module Day16

include("InputHelper.jl")

function get_input()
    groups = InputHelper.parse(line -> line, "16", "\n\n")
    rules = [parse_rule(line) for line in split(groups[1], "\n")]


    my_ticket = [parse(Int, c) for c in split(split(groups[2], "\n")[2], ",")]

    other_tickets = [[parse(Int, c) for c in split(line, ",")] for line in split(groups[3], "\n")[2:end]]

    (rules, my_ticket, other_tickets)
end

function parse_rule(line)
    name, numbers = split(line, ": ")
    split_numbers = split(numbers, " or ")
    parsed_numbers = [parse(Int, number) for split_number in split_numbers for number in split(split_number, "-")]

    parsed_range_one = parsed_numbers[1]:parsed_numbers[2]
    parsed_range_two = parsed_numbers[3]:parsed_numbers[4]

    (name, parsed_range_one, parsed_range_two)
end

function solve()
    input = get_input()

    solve_part_one(input), solve_part_two(input)
end

function solve_part_one((rules, my_ticket, other_tickets))
    invalid_number_sum = 0
    for ticket in other_tickets
        invalid_numbers = filter(number -> !is_number_valid(number, rules), ticket)
        invalid_number_sum += sum(invalid_numbers)
    end

    invalid_number_sum
end

function is_number_valid(number, rules)
    for (_, rangeone, rangetwo) in rules
        if in(number, rangeone) || in(number, rangetwo)
            return true
        end
    end

    false
end


function solve_part_two((rules, my_ticket, other_tickets))
    valid_tickets = filter(ticket -> !any(n -> !is_number_valid(n, rules), ticket), other_tickets)

    valid_columns_per_rule = find_valid_columns(rules, valid_tickets)

    unique_columns_per_rule = determine_unique_columns(rules, valid_columns_per_rule)

    departure_keys = filter(key -> occursin("departure", key), keys(unique_columns_per_rule))

    prod([my_ticket[unique_columns_per_rule[key]] for key in departure_keys])
end

function find_valid_columns(rules, valid_tickets)
    valid_columns_per_rule = Dict()

    for (name, rangeone, rangetwo) in rules
        possible_columns = Set(1:length(valid_tickets[1]))

        for ticket in valid_tickets
            for (i, n) in enumerate(ticket)
                if !(in(n, rangeone) || in(n, rangetwo))
                    pop!(possible_columns, i, -1)
                end
            end
        end

        valid_columns_per_rule[name] = possible_columns

    end
    valid_columns_per_rule
end

function determine_unique_columns(rules, valid_columns_per_rule)
    unique_columns_per_rule = Dict()
    columns_taken = []

    while length(unique_columns_per_rule) != length(rules)
        for key in keys(valid_columns_per_rule)
            valid_columns = valid_columns_per_rule[key]

            for n in columns_taken
                pop!(valid_columns, n, 0)
            end

            if length(valid_columns) == 1
                column = pop!(valid_columns)

                push!(columns_taken, column)

                unique_columns_per_rule[key] = column
                delete!(valid_columns_per_rule, key)
            end
        end
    end

    unique_columns_per_rule
end

end
