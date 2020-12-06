module InputHelper

using HTTP
import DotEnv


function download(day::String)
    cfg = DotEnv.config()
    session_cookie = cfg["AOC_SESSION"]
    root_url = "https://adventofcode.com/2020/day"

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
        println("Do you have your session cookie (AOC_SESSION) in .env?")
    end
end

function parse(f::Function, day::String, split_by::String="\n")::Array{Any,1}
    input_path = joinpath(@__DIR__, "../inputs/day$day")
    input = []

    open(input_path, "r") do io
        input = read(io, String)
    end

    map(f, split(strip(input), split_by))
end
end
