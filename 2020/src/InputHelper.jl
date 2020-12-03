module InputHelper

using HTTP

session_cookie = ""

root_url = "https://adventofcode.com/2020/day"
function download(day::String)
    cookies = Dict("session" => session_cookie)
    fetch_url = "$root_url/$day/input"
    r = HTTP.get(fetch_url; cookies=cookies)

    out_file_path = joinpath(@__DIR__, "../inputs/day$day")
    if r.status == 200
        open(out_file_path, "w") do io
            write(io, String(r.body))
        end
    else
        println(r.status)
        println(String(r.body))
    end
end

function parse(day::String, f::Function)::Array{Any,1}
    input_path = joinpath(@__DIR__, "../inputs/day$day")
    input = []

    open(input_path, "r") do io
        for line in readlines(io)
            if line == ""
                continue
            end
            push!(input, f(line))
        end
    end
    input
end
end
