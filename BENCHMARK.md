# Benchmarks

> Here you can see the efficiency results for this project.

Machine in action is `MacBook Pro M2 Max with 32 GB memory`.
Hardware impacts only `Execution Time` and `Proving Time`.

_Note: time results may diverge between launches with a low delta._

| Duplicates | Leaves | Potential Duplicates Supplied | Number of Segments | Total Cycles | User Cycles | Execution Time  | Proving Time       |
|------------|--------|-------------------------------|--------------------|--------------|-------------|-----------------|--------------------|
| 2          | 4      | 2                             | 1                  | 131072       | 43287       | 2.732792ms      | 6.680840083s       |
| 2          | 4      | 3                             | 1                  | 262144       | 55230       | 3.126958ms      | 13.45861175s       |
| 3          | 4      | 4                             | 1                  | 262144       | 67310       | 3.30175ms       | 14.467034708s      |
| 2          | 6      | 2                             | 1                  | 262144       | 82238       | 4.228333ms      | 13.900886458s      |
| 3          | 6      | 5                             | 1                  | 262144       | 97965       | 4.661208ms      | 13.596875833s      |
| 6          | 6      | 6                             | 1                  | 262144       | 114530      | 5.072042ms      | 13.487512667s      |
| 3          | 24     | 7                             | 1                  | 524288       | 184310      | 6.9675ms        | 26.112994875s      |
| 24         | 24     | 24                            | 1                  | 1048576      | 588269      | 18.734333ms     | 53.704900834s      |
| 17         | 48     | 19                            | 1                  | 1048576      | 541322      | 17.398542ms     | 53.923449958s      |
| 48         | 48     | 48                            | 1                  | 2097152      | 1340426     | 39.9815ms       | 109.094772167s     |
| 187        | 256    | 199                           | 9                  | 9437184      | 7005508     | 202.022917ms    | 498.424422667s     |
| 589        | 1000   | 600                           | 34                 | 35127296     | 25865248    | 737.140916ms    | 1837.905698417s    |
