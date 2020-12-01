#pragma once

#include <array>
#include <chrono>
#include <deque>
#include <iostream>
#include <thread>
#include <vector>

namespace aoc::intcode {

using program_t = std::vector<long long int>;
using channel = std::deque<long long int>;

static std::array<int, 4> get_parameters(int instruction) {
  int op_code = instruction % 100;
  int mode_c = int(instruction / 100) % 10;
  int mode_b = int(instruction / 1000) % 10;
  int mode_a = int(instruction / 10000) % 10;
  return {mode_a, mode_b, mode_c, op_code};
}

size_t get_address(program_t &program, size_t program_pointer, int mode,
                   size_t relative_base) {
  size_t output = 0;
  if (program_pointer >= program.size()) {
    program.resize(program_pointer + 1, 0);
  }

  if (mode == 0) {
    output = program[program_pointer];
  } else if (mode == 1) {
    output = program_pointer;
  } else {
    output = relative_base + program[program_pointer];
  }

  if (output >= program.size()) {
    program.resize(output + 1, 0);
  }
  return output;
}

void add(program_t &program, size_t &program_pointer, size_t relative_base,
         int mode_a, int mode_b, int mode_c) {
  size_t address_one =
      get_address(program, program_pointer + 1, mode_c, relative_base);
  size_t address_two =
      get_address(program, program_pointer + 2, mode_b, relative_base);

  size_t output =
      get_address(program, program_pointer + 3, mode_a, relative_base);

  if (mode_a == 1) {
    throw std::invalid_argument("ADD: WRONG MODE FOR OUTPUT");
  }

  program[output] = program[address_one] + program[address_two];
  program_pointer += 4;
}

void mul(program_t &program, size_t &program_pointer, size_t relative_base,
         int mode_a, int mode_b, int mode_c) {
  size_t address_one =
      get_address(program, program_pointer + 1, mode_c, relative_base);
  size_t address_two =
      get_address(program, program_pointer + 2, mode_b, relative_base);
  size_t output =
      get_address(program, program_pointer + 3, mode_a, relative_base);

  if (mode_a == 1) {
    throw std::invalid_argument("MUL: WRONG MODE FOR OUTPUT");
  }

  program[output] = program[address_one] * program[address_two];
  program_pointer += 4;
}

void input(program_t &program, size_t &program_pointer, size_t relative_base,
           channel &inputs, int mode_c) {

  while (inputs.empty()) {
    using namespace std::chrono_literals;
//        std::cout << "Give input: " << std::endl;
//        std::cin >> val;
    std::this_thread::sleep_for(10us);
  }

  auto val = inputs.front();
  inputs.pop_front();

  size_t output =
      get_address(program, program_pointer + 1, mode_c, relative_base);

  if (mode_c == 1) {
    throw std::invalid_argument("INPUT: WRONG MODE FOR OUTPUT");
  }

  program[output] = val;
  program_pointer += 2;
}

void output(program_t &program, size_t &program_pointer, size_t relative_base,
            channel &outputs, int mode_c) {
  size_t address_one =
      get_address(program, program_pointer + 1, mode_c, relative_base);

  outputs.push_back(program[address_one]);
//  std::cout << program[address_one] << std::endl;
  program_pointer += 2;
}

void jump_neq(program_t &program, size_t &program_pointer, size_t relative_base,
              int mode_b, int mode_c) {
  size_t address_one =
      get_address(program, program_pointer + 1, mode_c, relative_base);
  size_t address_two =
      get_address(program, program_pointer + 2, mode_b, relative_base);

  if (program[address_one] != 0) {
    program_pointer = program[address_two];
  } else {
    program_pointer += 3;
  }
}

void jump_eq(program_t &program, size_t &program_pointer, size_t relative_base,
             int mode_b, int mode_c) {
  size_t address_one =
      get_address(program, program_pointer + 1, mode_c, relative_base);
  size_t address_two =
      get_address(program, program_pointer + 2, mode_b, relative_base);

  if (program[address_one] == 0) {
    program_pointer = program[address_two];
  } else {
    program_pointer += 3;
  }
}

void less_than(program_t &program, size_t &program_pointer,
               size_t relative_base, int mode_a, int mode_b, int mode_c) {
  size_t address_one =
      get_address(program, program_pointer + 1, mode_c, relative_base);
  size_t address_two =
      get_address(program, program_pointer + 2, mode_b, relative_base);
  size_t output =
      get_address(program, program_pointer + 3, mode_a, relative_base);

  if (mode_a == 1) {
    throw std::invalid_argument("LESS THAN: WRONG MODE FOR OUTPUT");
  }

  if (program[address_one] < program[address_two]) {
    program[output] = 1;
  } else {
    program[output] = 0;
  }
  program_pointer += 4;
}

void equals(program_t &program, size_t &program_pointer, size_t relative_base,
            int mode_a, int mode_b, int mode_c) {
  size_t address_one =
      get_address(program, program_pointer + 1, mode_c, relative_base);
  size_t address_two =
      get_address(program, program_pointer + 2, mode_b, relative_base);
  size_t output =
      get_address(program, program_pointer + 3, mode_a, relative_base);
  if (mode_a == 1) {
    throw std::invalid_argument("EQUALS: WRONG MODE FOR OUTPUT");
  }

  if (program[address_one] == program[address_two]) {
    program[output] = 1;
  } else {
    program[output] = 0;
  }
  program_pointer += 4;
}

void adjust_relative_base(program_t &program, size_t &program_pointer,
                          size_t &relative_base, int mode_c) {
  size_t address_one =
      get_address(program, program_pointer + 1, mode_c, relative_base);

  relative_base += program[address_one];

  program_pointer += 2;
}

void run_instruction(program_t &program, size_t &program_pointer,
                     size_t &relative_base, channel &inputs, channel &outputs) {
  auto [mode_a, mode_b, mode_c, op_code] =
      get_parameters(program[program_pointer]);

  if (op_code == 1) {
    add(program, program_pointer, relative_base, mode_a, mode_b, mode_c);
  } else if (op_code == 2) {
    mul(program, program_pointer, relative_base, mode_a, mode_b, mode_c);
  } else if (op_code == 3) {
    input(program, program_pointer, relative_base, inputs, mode_c);
  } else if (op_code == 4) {
    output(program, program_pointer, relative_base, outputs, mode_c);
  } else if (op_code == 5) {
    jump_neq(program, program_pointer, relative_base, mode_b, mode_c);
  } else if (op_code == 6) {
    jump_eq(program, program_pointer, relative_base, mode_b, mode_c);
  } else if (op_code == 7) {
    less_than(program, program_pointer, relative_base, mode_a, mode_b, mode_c);
  } else if (op_code == 8) {
    equals(program, program_pointer, relative_base, mode_a, mode_b, mode_c);
  } else if (op_code == 9) {
    adjust_relative_base(program, program_pointer, relative_base, mode_c);
  } else {
    throw std::invalid_argument("No such opcode " + std::to_string(op_code));
  }
}

static void run_program(program_t &program) {
  size_t program_pointer = 0;
  size_t relative_base = 0;
  channel inputs{};
  channel outputs{};

  while (program[program_pointer] != 99) {
    intcode::run_instruction(program, program_pointer, relative_base, inputs,
                             outputs);
  }
}

static void run_program(program_t &program, channel &inputs, channel &outputs) {
  size_t program_pointer = 0;
  size_t relative_base = 0;

  while (program[program_pointer] != 99) {
    intcode::run_instruction(program, program_pointer, relative_base, inputs,
                             outputs);
  }
}

static void run_program_thread(program_t program, channel &inputs,
                               channel &outputs) {
  run_program(program, inputs, outputs);
  outputs.push_back(-1);
}

} // namespace aoc::intcode