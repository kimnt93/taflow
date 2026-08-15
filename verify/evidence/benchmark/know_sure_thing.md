# KnowSureThing benchmark (`KST` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.16M | 0.018 | 54.32M | 0.694 | 36.17× | 37.68× |
| 10,000 | 0.171 | 58.33M | 0.173 | 57.91M | 3.574 | 20.85× | 20.70× |
| 100,000 | 1.743 | 57.39M | 1.737 | 57.58M | 36.053 | 20.69× | 20.76× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.621 | 7.07× |
| 1 | 5 | 0.328 | 2.535 | 7.72× |
| 1 | 10 | 0.439 | 4.924 | 11.22× |
| 10 | 1 | 0.049 | 0.469 | 9.61× |
| 10 | 5 | 0.202 | 2.500 | 12.37× |
| 10 | 10 | 0.438 | 5.040 | 11.52× |
| 100 | 1 | 0.049 | 0.496 | 10.14× |
| 100 | 5 | 0.204 | 2.700 | 13.25× |
| 100 | 10 | 0.436 | 5.309 | 12.19× |
| 1,000 | 1 | 0.064 | 1.050 | 16.38× |
| 1,000 | 5 | 0.207 | 4.678 | 22.58× |
| 1,000 | 10 | 0.453 | 8.958 | 19.80× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
