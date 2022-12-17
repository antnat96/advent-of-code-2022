def main():
    highest = 0
    this_elf = []
    with open('input.txt') as input_file:
        for line in input_file:
            if line == "\n":
                if sum(this_elf) > highest:
                    highest = sum(this_elf)
                this_elf = []
            else:
                this_elf.append(int(line.rstrip()))
    print(highest)
  

if __name__ == '__main__':
  main()