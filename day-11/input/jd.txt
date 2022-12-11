Monkey 0:
  Starting items: 74, 64, 74, 63, 53
  Operation: new = old * 7
  Test: divisible by 5
    If true: throw to monkey 1
    If false: throw to monkey 6

Monkey 1:
  Starting items: 69, 99, 95, 62
  Operation: new = old * old
  Test: divisible by 17
    If true: throw to monkey 2
    If false: throw to monkey 5

Monkey 2:
  Starting items: 59, 81
  Operation: new = old + 8
  Test: divisible by 7
    If true: throw to monkey 4
    If false: throw to monkey 3

Monkey 3:
  Starting items: 50, 67, 63, 57, 63, 83, 97
  Operation: new = old + 4
  Test: divisible by 13
    If true: throw to monkey 0
    If false: throw to monkey 7

Monkey 4:
  Starting items: 61, 94, 85, 52, 81, 90, 94, 70
  Operation: new = old + 3
  Test: divisible by 19
    If true: throw to monkey 7
    If false: throw to monkey 3

Monkey 5:
  Starting items: 69
  Operation: new = old + 5
  Test: divisible by 3
    If true: throw to monkey 4
    If false: throw to monkey 2

Monkey 6:
  Starting items: 54, 55, 58
  Operation: new = old + 7
  Test: divisible by 11
    If true: throw to monkey 1
    If false: throw to monkey 5

Monkey 7:
  Starting items: 79, 51, 83, 88, 93, 76
  Operation: new = old * 3
  Test: divisible by 2
    If true: throw to monkey 0
    If false: throw to monkey 6
