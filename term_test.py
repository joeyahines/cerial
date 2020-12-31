import sys
import serial


def tx_test(dev):
    with open(dev, "w") as f:
        for i in range(0, 100):
            f.write("\033[93m{}: This is a test message.\r\n".format(i))


def rx_test(ser: serial.Serial):
    while True:
        c = ser.read(1)
        print(c)
        ser.write(c)
        ser.flush()


def main():
    if len(sys.argv) < 3:
        print("{} <dev> <test>".format(sys.argv[0]))
        exit(-1)
    else:
        dev = sys.argv[1]
        test = sys.argv[2]
        ser = serial.Serial(dev)

        try:
            if test == "rx":
                rx_test(ser)
            else:
                tx_test(dev)
        finally:
            ser.close()


if __name__ == "__main__":
    main()
