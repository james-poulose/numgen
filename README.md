A CLI tool to generate popular numerical series.

1. [Fibonacci Series](#fibonacci-series)
2. [Tribonacci Series](#tribonacci-series)
3. [Nnacci Series](#nnacci-series)
4. [Prime Numbers](#nnacci-series)

##### Standard Options / Arguments

| Short | Long        | Description                                   |
| ----- | ----------- | --------------------------------------------- |
| -l    | --log       | Log file for output.                          |
| -v    | --verbosity | Verbosity level for logging. [default: Info]. |
| -h    | --help      | Print help.                                   |
| -V    | --version   | Print version.                                |

##### Fibonacci Series

The Fibonacci sequence is a sequence in which each number is the sum of the two preceding ones.

```
fibonacci [OPTIONS]
```

| Short | Long    | Description                                         |
| ----- | ------- | --------------------------------------------------- |
| -s    | --start | Start value (instead of 0). [default: 0]            |
| -c    | --count | Total numbers that will be generated. [default: 10] |

##### Tribonacci Series

The Tribonacci series is a generalization of the Fibonacci sequence where each term is the sum of the three preceding terms.

```
tribonacci [OPTIONS]
```

| Short | Long    | Description                                         |
| ----- | ------- | --------------------------------------------------- |
| -c    | --count | Total numbers that will be generated. [default: 10] |

##### Nnacci Series

The Nnacci series is a generalization of the Fibonacci & Tribonacci sequence where each term is the sum of the 'N' preceding terms.

```
nnacci [OPTIONS]
```

| Short | Long        | Description                                                                                             |
| ----- | ----------- | ------------------------------------------------------------------------------------------------------- |
| -c    | --count     | Total numbers that will be generated. [default: 10]                                                     |
| -d    | --dimension | The dimension of the Nnachi series (Fibonacci and Tribonacci have a dimension of 2 and 3 respectively). |

##### Prime Numbers

Prime numbers are natural numbers that are divisible by only 1 and the number itself.

```
prime [OPTIONS]
```

| Short | Long    | Description                                         |
| ----- | ------- | --------------------------------------------------- |
| -s    | --start | Start value (instead of 0). [default: 0]            |
| -c    | --count | Total numbers that will be generated. [default: 10] |
