module Day19

include("InputHelper.jl")

function get_input()
    groups =  InputHelper.parse(line -> line, "19", "\n\n")
    split_rules = [split(rule, ": ") for rule in split(groups[1], "\n")]
    rules = Dict(parse(Int, idx) => parse_rule(rule) for (idx, rule) in split_rules)

    messages = split(groups[2], "\n")

    (rules, messages)
end

function parse_rule(rule)
    if rule == "\"a\""
        return 'a'
    elseif rule == "\"b\""
        return 'b'
    else
        subrules = split(rule, " | ")
        return [[parse(Int, i) for i in split(subrule, " ")] for subrule in subrules]
    end
end

function solve()
    input = get_input()

    solve_part_one(input), solve_part_two(input)
end

function solve_part_one((rules, messages))
    n = 0
    for m in messages
        matches, last_character_matched = matches_rule(m, 1, rules[0], rules)
        if any(l == length(m) for l in last_character_matched) && matches
            n += 1
        end
    end
    n
end

function matches_rule(message, idx,  rule, rules)
    if idx > length(message)
        return false, nothing
    end

    if rule == 'a' || rule == 'b'
        return message[idx] == rule, [idx]
    else
        possible_subrules = (match_subrule(message, idx, subrule, rules) for subrule in rule)

        all_matching_indicies = vcat([indicies for (match, indicies) in possible_subrules if match]...)

        return length(all_matching_indicies) != 0, all_matching_indicies
    end

    false, nothing
end

function match_subrule(message, idx, subrule, rules)
    sub_idxes = [idx - 1]

    for sub in subrule
        possible_matches = (matches_rule(message, sub_idx + 1, rules[sub], rules) for sub_idx in sub_idxes)

        sub_idxes = vcat([array for (match, array) in possible_matches if match]...)

        if length(sub_idxes) == 0
            return false, nothing
        end
    end

    true, sub_idxes
end

function ugly_deal_with_repeats(rule_idx, rule)
    if length(rule) == 1
        return rule
    elseif !in(rule_idx, rule[2])
        return rule
    else
        first_subrule = rule[1]
        new_subrules = [first_subrule]
        for i in 1:10
            new_subrule = []
            for r in rule[2]
                if r != rule_idx
                    push!(new_subrule, r)
                else
                    for sub in first_subrule
                        append!(new_subrule, repeat([sub], i))
                    end
                end
            end
            push!(new_subrules, new_subrule)
        end
        return new_subrules
    end

end

function solve_part_two((rules, messages))
    rules[8] = ugly_deal_with_repeats(8, parse_rule("42 | 42 8"))
    rules[11] = ugly_deal_with_repeats(11, parse_rule("42 31 | 42 11 31"))
    n = 0
    for m in messages
        matches, last_character_matched = matches_rule(m, 1, rules[0], rules)
        # println("$m $matches")
        if any(l == length(m) for l in last_character_matched) && matches
            n += 1
        end
    end
    n
end

end
