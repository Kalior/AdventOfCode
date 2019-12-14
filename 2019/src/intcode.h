#include <array>
#include <iostream>
#include <vector>
#include <deque>
#include <chrono>
#include <thread>

namespace aoc::intcode {

static std::array<int, 4> get_parameters(int instruction) {
  int op_code = instruction % 100;
  int mode_c = int(instruction / 100) % 10;
  int mode_b = int(instruction / 1000) % 10;
  int mode_a = int(instruction / 10000) % 10;
  return {mode_a, mode_b, mode_c, op_code};
}

int get_value(std::vector<int> &program, int program_pointer, int mode) {
  if (mode == 0) {
    return program.at(program.at(program_pointer));
  } else {
    return program.at(program_pointer);
  }
}

int add(std::vector<int> &program, int program_pointer, int mode_b,
        int mode_c) {
  int value_one = get_value(program, program_pointer + 1, mode_c);
  int value_two = get_value(program, program_pointer + 2, mode_b);
  int output = program.at(program_pointer + 3);
  program.at(output) = value_one + value_two;
  return program_pointer + 4;
}

int mul(std::vector<int> &program, int program_pointer, int mode_b,
        int mode_c) {
  int value_one = get_value(program, program_pointer + 1, mode_c);
  int value_two = get_value(program, program_pointer + 2, mode_b);
  int output = program.at(program_pointer + 3);
  program.at(output) = value_one * value_two;
  return program_pointer + 4;
}

int input(std::vector<int> &program, int program_pointer,
          std::deque<int> &inputs) {

  while (inputs.empty()) {
    using namespace std::chrono_literals;
    std::this_thread::sleep_for(10us);
//    std::cout << "Give input: " << std::endl;
//    std::cin >> val;
  }

  int val = inputs.front();
  inputs.pop_front();

  int output = program.at(program_pointer + 1);
  program.at(output) = val;
  return program_pointer + 2;
}

int output(std::vector<int> &program, int program_pointer,
           std::deque<int> &outputs, int mode_c) {
  int value_one = get_value(program, program_pointer + 1, mode_c);
  outputs.push_back(value_one);
//  std::cout << value_one << std::endl;
  return program_pointer + 2;
}

int jump_neq(std::vector<int> &program, int program_pointer, int mode_b,
             int mode_c) {
  int value_one = get_value(program, program_pointer + 1, mode_c);
  int value_two = get_value(program, program_pointer + 2, mode_b);
  if (value_one != 0) {
    return value_two;
  } else {
    return program_pointer + 3;
  }
}

int jump_eq(std::vector<int> &program, int program_pointer, int mode_b,
            int mode_c) {
  int value_one = get_value(program, program_pointer + 1, mode_c);
  int value_two = get_value(program, program_pointer + 2, mode_b);
  if (value_one == 0) {
    return value_two;
  } else {
    return program_pointer + 3;
  }
}

int less_than(std::vector<int> &program, int program_pointer, int mode_b,
              int mode_c) {
  int value_one = get_value(program, program_pointer + 1, mode_c);
  int value_two = get_value(program, program_pointer + 2, mode_b);
  int output = program.at(program_pointer + 3);
  if (value_one < value_two) {
    program.at(output) = 1;
  } else {
    program.at(output) = 0;
  }
  return program_pointer + 4;
}

int equals(std::vector<int> &program, int program_pointer, int mode_b,
           int mode_c) {
  int value_one = get_value(program, program_pointer + 1, mode_c);
  int value_two = get_value(program, program_pointer + 2, mode_b);
  int output = program.at(program_pointer + 3);
  if (value_one == value_two) {
    program.at(output) = 1;
  } else {
    program.at(output) = 0;
  }
  return program_pointer + 4;
}

int run_instruction(std::vector<int> &program, int program_pointer,
                    std::deque<int> &inputs, std::deque<int> &outputs) {
  auto [mode_a, mode_b, mode_c, op_code] =
      get_parameters(program.at(program_pointer));

  if (op_code == 1) {
    program_pointer = add(program, program_pointer, mode_b, mode_c);
  } else if (op_code == 2) {
    program_pointer = mul(program, program_pointer, mode_b, mode_c);
  } else if (op_code == 3) {
    program_pointer = input(program, program_pointer, inputs);
  } else if (op_code == 4) {
    program_pointer = output(program, program_pointer, outputs, mode_c);
  } else if (op_code == 5) {
    program_pointer = jump_neq(program, program_pointer, mode_b, mode_c);
  } else if (op_code == 6) {
    program_pointer = jump_eq(program, program_pointer, mode_b, mode_c);
  } else if (op_code == 7) {
    program_pointer = less_than(program, program_pointer, mode_b, mode_c);
  } else if (op_code == 8) {
    program_pointer = equals(program, program_pointer, mode_b, mode_c);
  } else {
    throw std::invalid_argument("No such opcode " + std::to_string(op_code));
  }

  return program_pointer;
}

int run_instruction(std::vector<int> &program, int program_pointer) {
  std::deque<int> inputs{};
  std::deque<int> outputs{};
  return run_instruction(program, program_pointer, inputs, outputs);
}

} // namespace aoc::intcode