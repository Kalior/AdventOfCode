using AdventOfCode2020
using Test

include("../src/day1.jl")
include("../src/day2.jl")

@testset "AdventOfCode2020.jl" begin
    @testset "Day one" begin
        @test Day1.sums_to_2020(2020, 0)
        @test Day1.sums_to_2020(2018, 0, 2)
        @test !Day1.sums_to_2020(2018, 0, 3)
        @test Day1.sums_to_2020(1010, 1010)

        input = Day1.get_input()
        @test Day1.solve_part_one(input) == 471019
        @test Day1.solve_part_two(input) == 103927824
    end

    @testset "Day two" begin
        @test Day2.valid_passphrase((1, 3, 'a', "abcde"))
        @test !Day2.valid_passphrase((1, 3, 'b', "cdefg"))
        @test Day2.valid_passphrase((2, 9, 'c', "ccccccccc"))

        @test Day2.valid_passphrase_two((1, 3, 'a', "abcde"))
        @test !Day2.valid_passphrase_two((1, 3, 'b', "cdefg"))
        @test !Day2.valid_passphrase_two((2, 9, 'c', "ccccccccc"))

        input = Day2.get_input()
        @test Day2.solve_part_one(input) == 625
        @test Day2.solve_part_two(input) == 391
    end
end
