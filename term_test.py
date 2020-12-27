import sys


def main():
    if len(sys.argv) < 2:
        print("{} <dev>".format(sys.argv[0]))
        exit(-1)
    else:
        dev = sys.argv[1]

        with open(dev, "w") as f:
            for i in range(0, 100):
                f.write("\033[93m{}: This is a test message.\r\n".format(i))


if __name__ == "__main__":
    main()
