import sys
from time import perf_counter

from aoc2024 import *

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
    tasks = {i: globals()[f"Day{i}"] for i in range(1, 13)}


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
    t = perf_counter()
    part1 = task.part1(input)
    
    elapsed_time = (perf_counter() - t)*1e3
    print(f"{part1}\n in {elapsed_time:.1f}ms")
    t = perf_counter()
    part2 = task.part2(input)
    elapsed_time = (perf_counter() - t)*1e3
    print(f"{part2}\n in {elapsed_time:.1f}ms")

if __name__ == "__main__":
    main(2024)