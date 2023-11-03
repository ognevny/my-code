# a list of useful functions, like it did in dad-is-great-in-C


def rev(n):
    rev = 0
    while n:
        rev = rev * 10 + (n % 10)
        n //= 10
    return rev


def numsys(n, k):
    ns = ""
    while n:
        ns += str(n % k)
        n //= k
    return int(ns[::-1])


def numsys16(n, k):
    nums = {
        0: "0",
        1: "1",
        2: "2",
        3: "3",
        4: "4",
        5: "5",
        6: "6",
        7: "7",
        8: "8",
        9: "9",
        10: "A",
        11: "B",
        12: "C",
        13: "D",
        14: "E",
        15: "F",
    }
    res = ""
    while n:
        res += nums[n % k]
        n //= k
    return res[::-1]
