# GoldenPocket benchmark (`GoldenPocket` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.19M | 0.017 | 59.37M | 0.496 | 25.86× | 29.42× |
| 10,000 | 0.139 | 72.09M | 0.139 | 71.89M | 3.935 | 28.37× | 28.29× |
| 100,000 | 1.399 | 71.46M | 1.289 | 77.56M | 41.804 | 29.87× | 32.42× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.184 | 0.309 | 1.68× |
| 1 | 5 | 0.279 | 0.835 | 2.99× |
| 1 | 10 | 0.475 | 8.367 | 17.63× |
| 10 | 1 | 0.052 | 0.175 | 3.35× |
| 10 | 5 | 0.226 | 0.859 | 3.80× |
| 10 | 10 | 0.511 | 2.000 | 3.91× |
| 100 | 1 | 0.054 | 0.211 | 3.93× |
| 100 | 5 | 0.242 | 1.272 | 5.25× |
| 100 | 10 | 0.504 | 2.345 | 4.65× |
| 1,000 | 1 | 0.068 | 0.661 | 9.68× |
| 1,000 | 5 | 0.244 | 3.258 | 13.35× |
| 1,000 | 10 | 0.528 | 6.317 | 11.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
