using Test

include("../src/day1.jl")

@testset "2021" begin
    @testset "Day one" begin
        input = Day1.parse()
        @test Day1.solve_part_one(input) == 1681
        @test Day1.solve_part_two(input) == 1704
    end
end
