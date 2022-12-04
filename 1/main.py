
def run():
    with open("input.txt") as f:
        input = f.read()
    elves = [sum([int(y) for y in x.splitlines()]) for x in input.split("\n\n")]
    elves.sort(reverse=True)
    print(sum(elves[i] for i in range(3)))



if __name__ == "__main__":
    run()
