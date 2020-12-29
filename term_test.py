import sys


def tx_test(dev):
    with open(dev, "w") as f:
        for i in range(0, 100):
            f.write("\033[93m{}: This is a test message.\r\n".format(i))


def rx_test(dev):
    with open(dev, "r") as f:
        while True:
            print(f.read())


def main():
    if len(sys.argv) < 3:
        print("{} <dev> <test>".format(sys.argv[0]))
        exit(-1)
    else:
        dev = sys.argv[1]
        test = sys.argv[2]

        if test == "rx":
            rx_test(dev)
        else:
            tx_test(dev)


if __name__ == "__main__":
    main()
