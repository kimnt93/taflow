# MedianPrice benchmark (`MEDPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 177.10M | 0.004 | 247.25M | 0.028 | 4.94× | 6.89× |
| 10,000 | 0.019 | 529.42M | 0.016 | 632.45M | 0.032 | 1.67× | 2.00× |
| 100,000 | 0.151 | 660.71M | 0.135 | 738.35M | 0.068 | 0.45× | 0.50× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | 0.125 | 1.40× |
| 1 | 5 | 0.319 | 0.444 | 1.39× |
| 1 | 10 | 0.467 | 0.874 | 1.87× |
| 10 | 1 | 0.051 | 0.090 | 1.77× |
| 10 | 5 | 0.224 | 0.413 | 1.84× |
| 10 | 10 | 0.451 | 0.869 | 1.93× |
| 100 | 1 | 0.047 | 0.086 | 1.82× |
| 100 | 5 | 0.220 | 0.407 | 1.85× |
| 100 | 10 | 0.437 | 0.872 | 1.99× |
| 1,000 | 1 | 0.052 | 0.083 | 1.61× |
| 1,000 | 5 | 0.231 | 0.422 | 1.83× |
| 1,000 | 10 | 0.474 | 0.932 | 1.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
