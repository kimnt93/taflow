# EvenBetterSinewave benchmark (`ebsw` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 121.11M | 0.007 | 136.31M | 11.676 | 1414.17× | 1591.55× |
| 10,000 | 0.058 | 171.29M | 0.057 | 176.20M | 111.938 | 1917.35× | 1972.31× |
| 100,000 | 0.569 | 175.89M | 0.580 | 172.37M | 1117.112 | 1964.87× | 1925.59× |
| 1,000,000 | 5.924 | 168.82M | 5.137 | 194.65M | 11493.466 | 1940.30× | 2237.18× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.172 | 2.23× |
| 1 | 5 | 0.453 | 0.806 | 1.78× |
| 1 | 10 | 0.494 | 1.507 | 3.05× |
| 10 | 1 | 0.048 | 0.151 | 3.16× |
| 10 | 5 | 0.221 | 0.724 | 3.28× |
| 10 | 10 | 0.473 | 1.482 | 3.13× |
| 100 | 1 | 0.046 | 0.984 | 21.44× |
| 100 | 5 | 0.221 | 4.762 | 21.56× |
| 100 | 10 | 0.465 | 9.671 | 20.78× |
| 1,000 | 1 | 0.057 | 11.472 | 200.65× |
| 1,000 | 5 | 0.371 | 76.777 | 206.96× |
| 1,000 | 10 | 1.328 | 161.080 | 121.25× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
