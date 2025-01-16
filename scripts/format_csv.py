import argparse
from pathlib import Path
import glob


def get_max_length_of_columns(lines: list[str]) -> dict[int, int]:
    max_length = {}
    for line in lines:
        row = line.split(",")
        for i in range(len(row)):
            row[i] = row[i].strip()
            if i not in max_length:
                max_length[i] = len(row[i])
            elif len(row[i]) > max_length[i]:
                max_length[i] = len(row[i])
    return max_length


def main(input):
    for file in input:
        print(f"Formatting {file}")
        with open(file, "r") as f:
            lines = f.readlines()
            if len(lines) <= 1:
                print(f"File {file} needs a header and at least one line.")
                print(f"Skipping file {file}")
                continue
            max_length = get_max_length_of_columns(lines)
            for i in range(len(lines)):
                row = lines[i].split(",")
                for j in range(len(row)):
                    row[j] = row[j].strip()
                    row[j] = row[j].ljust(max_length[j])
                lines[i] = ",".join(row)
        with open(file, "w") as f:
            f.write("\n".join(lines))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Format one or more CSV files from the data folder. It will input tabs infront of the commas so the file is more readable."
    )
    parser.add_argument(
        "--input",
        type=Path,
        help="The input file or files",
        nargs="+",
        default=glob.glob("../data/*.csv"),
    )
    args = parser.parse_args()

    args.input = [Path(x) for x in args.input]

    main(args.input)
