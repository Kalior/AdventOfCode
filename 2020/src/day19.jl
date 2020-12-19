module Day19

include("InputHelper.jl")

function get_input()
    groups =  InputHelper.parse(line -> line, "19", "\n\n")
    split_rules = [split(rule, ": ") for rule in split(groups[1], "\n")]
    rules = Dict(parse(Int, idx) => rule for (idx, rule) in split_rules)

    messages = split(groups[2], "\n")

    (rules, messages)
end

function solve()
    input = get_input()

    solve_part_one(input), solve_part_two(input)
end

function solve_part_one((rules, messages))
    n = 0
    for m in messages
        matches, last_character_matched = matches_rule(m, 1, (0, rules[0]), rules)
        if any(l == length(m) for l in last_character_matched) && matches
            n += 1
        end
    end
    n
end

function matches_rule(message, idx, (rule_idx, rule), rules)
    if idx > length(message)
        return false, [idx]
    end

    if rule == "\"a\""
        return message[idx] == 'a', [idx]
    elseif rule == "\"b\""
        return message[idx] == 'b', [idx]
    else
        subrules = split(rule, " | ")
        subrules = [[parse(Int, i) for i in split(subrule, " ")] for subrule in subrules]

        subrules = ugly_deal_with_repeats((rule_idx, rule), subrules)

        matches = false
        all_sub_idxs = []

        for subrule in subrules
            matches_subrule = true
            sub_idxes = [idx - 1]
            for sub in subrule
                match = true
                new_sub_idxes = []
                for sub_idx in sub_idxes
                    other_match, other_new_sub_idxes = matches_rule(message, sub_idx + 1, (sub, rules[sub]), rules)
                    if other_match
                        match = true
                        append!(new_sub_idxes, other_new_sub_idxes)
                    end
                end

                if !match
                    matches_subrule = false
                    break
                else
                    sub_idxes = new_sub_idxes
                end
            end

            if matches_subrule
                matches = true
                append!(all_sub_idxs, sub_idxes)
            end
        end

        return matches, all_sub_idxs
    end

    false, [0]
end

function ugly_deal_with_repeats((rule_idx, rule), subrules)
    if length(subrules) == 1
        return subrules
    elseif !in(rule_idx, subrules[2])
        return subrules
    else
        first_subrule = subrules[1]
        new_subrules = [first_subrule]
        for i in 1:100
            new_subrule = []
            for r in subrules[2]
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
    rules[8] = "42 | 42 8"
    rules[11] = "42 31 | 42 11 31"
    n = 0
    for m in messages
        matches, last_character_matched = matches_rule(m, 1, (0, rules[0]), rules)
        println("$m $matches")
        if any(l == length(m) for l in last_character_matched) && matches
            n += 1
        end
    end
    n
end

end
