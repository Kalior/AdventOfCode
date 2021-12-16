module Day16
using Statistics
using Base.Iterators

include("ReadHelper.jl")

abstract type Packet end

struct LiteralPacket <: Packet
    version::Int
    type::Int
    number::Int
end

struct OperatorPacket <: Packet
    version::Int
    type::Int
    length_type::Int
    subpackets::Array{Packet}
end

function parse_literal_packet(packet, current_i, version)
    number_string = []

    i = current_i + 6
    while parse(Int, packet[i]) != 0
        push!(number_string, packet[i+1:i+4])
        i += 5
    end
    push!(number_string, packet[i+1:i+4])
    number = parse(Int, join(number_string), base = 2)
    i += 5

    LiteralPacket(version, 4, number), i, i - current_i
end

function parse_operator_packet(packet, current_i, version, type_id)
    i = current_i + 6
    length_type = parse(Int, packet[i])
    if length_type == 0
        return parse_operator_packet_0(packet, current_i, version, type_id)
    elseif length_type == 1
        return parse_operator_packet_1(packet, current_i, version, type_id)
    end
end

function parse_operator_packet_0(packet, current_i, version, type_id)
    i = current_i + 7
    length_vs = packet[i:i+14]
    n_total_bits = parse(Int, length_vs, base = 2)

    sub_i = i + 15
    child_packet_bits = 0
    subpackets = []
    while child_packet_bits < n_total_bits
        p, sub_i, packet_len = parse_packet(packet, sub_i)
        push!(subpackets, p)
        child_packet_bits += packet_len
    end

    OperatorPacket(version, type_id, 0, subpackets), sub_i, sub_i - current_i
end

function parse_operator_packet_1(packet, current_i, version, type_id)
    i = current_i + 7
    length_vs = packet[i:i+10]
    n_subpackets = parse(Int, length_vs, base = 2)

    sub_i = i + 11
    subpackets = []
    for _ = 1:n_subpackets
        p, sub_i, _ = parse_packet(packet, sub_i)
        push!(subpackets, p)
    end

    OperatorPacket(version, type_id, 1, subpackets), sub_i, sub_i - current_i
end

function parse_packet(packet, current_i)
    packet_vs = packet[current_i:current_i+2]
    packet_type_id = packet[current_i+3:current_i+5]

    version = parse(Int, packet_vs, base = 2)
    type_id = parse(Int, packet_type_id, base = 2)

    if type_id == 4
        return parse_literal_packet(packet, current_i, version)
    else
        return parse_operator_packet(packet, current_i, version, type_id)
    end
end

function parse_input()
    packet = first(ReadHelper.getInputMap(line -> line, "16", "\n"))
    converter = Dict(
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
    )


    packet = join([converter[c] for c in packet])
    p, _, _ = parse_packet(packet, 1)
    p
end

function solve()
    packet = parse_input()

    (solve_part_one(packet), solve_part_two(packet))
end


function solve_part_one(packet::Packet)
    version_sum = 0
    packets_to_check::Array{Packet} = [packet]
    while !isempty(packets_to_check)
        p = pop!(packets_to_check)
        version_sum += p.version

        if typeof(p) == OperatorPacket
            push!(packets_to_check, p.subpackets...)
        end
    end

    version_sum
end

function solve_part_two(packet)::Int
    calculate_packet(packet)
end

function calculate_packet(packet::Packet)::Float64
    if packet.type == 4
        return packet.number
    elseif packet.type == 0
        return sum(calculate_packet(p) for p in packet.subpackets)
    elseif packet.type == 1
        return prod(calculate_packet(p) for p in packet.subpackets)
    elseif packet.type == 2
        return minimum(calculate_packet(p) for p in packet.subpackets)
    elseif packet.type == 3
        return maximum(calculate_packet(p) for p in packet.subpackets)
    elseif packet.type == 5
        return calculate_packet(packet.subpackets[1]) > calculate_packet(packet.subpackets[2])
    elseif packet.type == 6
        return calculate_packet(packet.subpackets[1]) < calculate_packet(packet.subpackets[2])
    elseif packet.type == 7
        return calculate_packet(packet.subpackets[1]) == calculate_packet(packet.subpackets[2])
    end
end

end

Day16.solve()
