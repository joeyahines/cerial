import sys
import serial


def tx_test(ser: serial.Serial):
    """
    Simple TX test

    :param ser: device to send data to
    """
    for i in range(0, 100):
        ser.write("\033[93m{}: This is a test message.\r\n".format(i))


def rx_test(ser: serial.Serial):
    """
    Simple echo test

    :param ser: serial port to echo data on
    :return:
    """

    while True:
        c = ser.read(1)
        print(c)
        ser.write(c)
        ser.flush()


def main():
    """
    Main function
    """

    if len(sys.argv) < 3:
        # Print help message
        print("{} <dev> <test>".format(sys.argv[0]))
        exit(-1)
    else:
        # Parse args
        dev = sys.argv[1]
        test = sys.argv[2]
        ser = serial.Serial(dev)

        # Begin test based on arguments
        try:
            if test == "rx":
                rx_test(ser)
            else:
                tx_test(dev)
        finally:
            ser.close()


if __name__ == "__main__":
    main()
