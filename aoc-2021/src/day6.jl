using Base.Iterators

module Day6
include("ReadHelper.jl")


function parse_input()::Array{Int,1}
    ReadHelper.getInputMap(f -> parse(Int, f), "6", ",")
end

function solve()
    input = parse_input()

    (solve_part_one(input), solve_part_two(input))
end

function solve_part_one(fishes)
    simulate_growth(fishes, 80)
end

function simulate_growth(fishes, n_days)
    fish_at_day = zeros(9)
    for fish in fishes
        fish_at_day[fish+1] += 1
    end

    for _ in range(1, stop = n_days)
        new_fish_at_day = zeros(9)

        fresh_fishes = fish_at_day[1]

        new_fish_at_day[9] = fresh_fishes
        new_fish_at_day[7] = fresh_fishes

        for i in range(1, stop = 9)
            new_fish_at_day[i] += get(fish_at_day, i + 1, 0)
        end
        fish_at_day = new_fish_at_day

    end

    Int64(sum(fish_at_day))
end

function solve_part_two(fishes)
    simulate_growth(fishes, 256)
end

end
