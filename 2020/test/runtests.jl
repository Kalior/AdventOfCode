using AdventOfCode2020
using Test

include("../src/day1.jl")

@testset "AdventOfCode2020.jl" begin
    @testset "Day one" begin
        @test sums_to_2020(2020, 0)
        @test sums_to_2020(2018, 0, 2)
        @test !sums_to_2020(2018, 0, 3)
        @test sums_to_2020(1010, 1010)

        input = get_input()
        @test solve_part_one(input) == 471019
        @test solve_part_two(input) == 103927824
    end
end
