module Day13

include("InputHelper.jl")

function get_input()::Array{String,1}
    InputHelper.parse(line -> line, "13", "\n")
end

function solve()

    input = get_input()

    solve_part_one(input), solve_part_two(input)
end

function solve_part_one(input)
    earliest_time = 1005595
    busses = [41, 37, 557, 29, 13, 17, 23, 419, 19]

    time_to_wait = Inf
    out = 0
    for buss in busses
        bus_wait = wait_time(buss, earliest_time)
        if bus_wait < time_to_wait
            time_to_wait = bus_wait
            out = buss * time_to_wait
        end
    end

    out
end

function wait_time(buss, start_time)::Int
    n_runs = ceil(start_time / buss)
    earliest_depart = n_runs * buss
    earliest_depart - start_time
end

function time_to_sequential(busses)
    intervals = [(index - 1) for (index, val) in enumerate(busses) if val !== nothing]
    real_busses = filter(buss -> buss !== nothing, busses)

    end_  = 1000000000000000000000000000

    periods = intersect([-interval:buss:end_ for (buss, interval) in zip(real_busses, intervals)]...)

    # Check for validity, the first one really should be okay
    for period in periods
        # periods can be more than one away
        if all(any(wait_time(buss, period) + (i * buss) == wait  for i in 0:1000) for (buss, wait) in zip(real_busses, intervals))
            return period
        end
    end
    return nothing
end

function solve_part_two(input)
    busses = [41,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,37,nothing,nothing,nothing,nothing,nothing,557,nothing,29,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,13,nothing,nothing,nothing,17,nothing,nothing,nothing,nothing,nothing,23,nothing,nothing,nothing,nothing,nothing,nothing,nothing,419,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,nothing,19]
    time_to_sequential(busses)
end

end
