import sys
from time import process_time_ns

from day1 import Day1
from day2 import Day2
from day3 import Day3
from day4 import Day4
from day5 import Day5
from day6 import Day6
from day7 import Day7
from day8 import Day8
from day9 import Day9
from day10 import Day10

# Add more day functions as needed
def read_input_file(day: int, year: int) -> str:
    # Construct the file path based on day and year
    file_path = f"./inputs/aoc{year}/input_day{day}.txt"
    
    try:
        # Open and read the file
        with open(file_path, 'r') as file:
            content = file.read()
        return content
    except FileNotFoundError:
        return f"Error: File not found at {file_path}"
    except Exception as e:
        return f"Error: {e}"
    
def main(year):
    # Dispatch table
    tasks = {
        1: Day1,
        2: Day2,
        3: Day3,
        4: Day4,
        5: Day5,
        6: Day6,
        7:Day7,
        8:Day8,
        9:Day9,
        10:Day10
        # Add more days here
    }

    if len(sys.argv) != 2:
        print("Usage: python main.py <day>")
        sys.exit(1)

    try:
        day = int(sys.argv[1])
        task = tasks.get(day)
        input = read_input_file(day,year)
        if task:
            print(f"Day {day}")
        else:
            print(f"Error: No task defined for day {day}.")
    except ValueError:
        print("Error: <day> must be an integer.")
    t = process_time_ns()
    part1 = task.part1(input)
    
    elapsed_time = (process_time_ns() - t)*10e-6
    print(f"{part1}\n in {elapsed_time:.3f}")
    t = process_time_ns()
    part2 = task.part2(input)
    elapsed_time = (process_time_ns() - t)*10e-6
    print(f"{part2}\n in {elapsed_time:.3f}")

if __name__ == "__main__":
    main(2024)