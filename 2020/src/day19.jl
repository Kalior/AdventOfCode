module Day19

include("InputHelper.jl")

function get_input()::Tuple{Dict{Int,Rule},Array{String}}
    groups =  InputHelper.parse(line -> line, "19", "\n\n")
    split_rules = [split(rule, ": ") for rule in split(groups[1], "\n")]
    rules = Dict(parse(Int, idx) => parse_rule(rule) for (idx, rule) in split_rules)

    messages = split(groups[2], "\n")

    (rules, messages)
end

CharRule = Char
OrRule = Array{Array{Int,1},1}
Rule = Union{CharRule,OrRule}
function parse_rule(rule::AbstractString)::Rule
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

function solve_part_one((rules, messages)::Tuple{Dict{Int,Rule},Array{String}})::Int
    n = 0
    for m in messages
        last_characters_matched = matches_rule(m, 1, rules[0], rules)
        if any(l == length(m) for l in last_characters_matched) && length(last_characters_matched) != 0
            n += 1
        end
    end
    n
end


function matches_rule(message::String, idx::Int, rule::CharRule, rules::Dict{Int,Rule})::Array{Int}
    if idx > length(message) || message[idx] != rule
        return []
    elseif message[idx] == rule
        return [idx]
    end
end

function matches_rule(message::String, idx::Int, rule::OrRule, rules::Dict{Int,Rule})::Array{Int}
    all_matching_indicies = vcat((match_subrule(message, idx, subrule, rules) for subrule in rule)...)

    return all_matching_indicies
end

function match_subrule(message::String, idx::Int, subrule::Array{Int,1}, rules::Dict{Int,Rule})::Array{Int}
    possible_matches = [idx - 1]

    for sub in subrule
        possible_matches = vcat((matches_rule(message, sub_idx + 1, rules[sub], rules) for sub_idx in possible_matches)...)

        if length(possible_matches) == 0
            return []
        end
    end

    possible_matches
end

function ugly_deal_with_repeats(rule_idx::Int, rule::OrRule)::OrRule
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

function solve_part_two((rules, messages)::Tuple{Dict{Int,Rule},Array{String}})::Int
    rules[8] = ugly_deal_with_repeats(8, parse_rule("42 | 42 8"))
    rules[11] = ugly_deal_with_repeats(11, parse_rule("42 31 | 42 11 31"))
    n = 0
    for m in messages
        last_characters_matched = matches_rule(m, 1, rules[0], rules)
        # println("$m $(length(last_characters_matched) == 0)")
        if any(l == length(m) for l in last_characters_matched) && length(last_characters_matched) != 0
            n += 1
        end
    end
    n
end

end
