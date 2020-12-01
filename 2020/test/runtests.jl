using AdventOfCode2020
using Test

include("../src/day1.jl")

@testset "AdventOfCode2020.jl" begin
    @testset "DayOne" begin
        @test sums_to_2020(2020, 0)
        @test sums_to_2020(2018, 0, 2)
        @test !sums_to_2020(2018, 0, 3)
        @test sums_to_2020(1010, 1010)
    end
end
