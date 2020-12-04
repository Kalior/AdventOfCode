module Day4

include("InputHelper.jl")

function get_input()::Array{Dict{String,String},1}
    pattern = r"(?<key>\S+):(?<value>\S+)"

    InputHelper.parse(line -> Dict(capture[:key] => capture[:value] for capture in eachmatch(pattern, line)), "4", "\n\n")
end

function solve()

    input = get_input()

    println("Part one $(solve_part_one(input))")

    println("Part two $(solve_part_two(input))")
end

function solve_part_one(passports::Array{Dict{String,String},1})
    count(valid_passport, passports)
end

function valid_passport(passport::Dict{String,String})::Bool
    required_fields = Dict("byr" => false, "iyr" => false, "eyr" => false, "hgt" => false, "hcl" => false, "ecl" => false, "pid" => false) # "cid"
    for (key, value) in passport
        required_fields[key] = true
    end

    all(values(required_fields))
end

check_birthyear(v) = 1920 <= parse(Int, v) <= 2002
check_issue_year(v) = 2010 <= parse(Int, v) <= 2020
check_expire_year(v) = 2020 <= parse(Int, v) <= 2030
check_colour(v) = match(r"^#[a-z0-9]{6}$", v) !== nothing
check_eyes(v) = occursin(v, join(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"], ","))
check_pid(v) = match(r"^[0-9]{9}$", v) !== nothing

function check_height(v)
    if occursin("cm", v)
        h = replace(v, "cm" => "")
        return 150 <= parse(Int, h) <= 193
    else
        h = replace(v, "in" => "")
        return 59 <= parse(Int, h) <= 76
    end
end

function valid_passport_with_security(passport::Dict{String,String})::Bool

    fields = Dict("byr" => check_birthyear, "iyr" => check_issue_year, "eyr" => check_expire_year, "hgt" => check_height, "hcl" => check_colour, "ecl" => check_eyes, "pid" => check_pid) # "cid"
    for (key, value) in passport
        if key == "cid"
            continue
        elseif !fields[key](value)
            return false
        end
    end

    return valid_passport(passport)
end

function solve_part_two(passports::Array{Dict{String,String},1})
    count(valid_passport_with_security, passports)
end

end
