# CandleInNeck benchmark (`CDLINNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 78.05M | 0.010 | 103.45M | 0.034 | 2.67× | 3.53× |
| 10,000 | 0.144 | 69.53M | 0.141 | 70.94M | 0.121 | 0.84× | 0.86× |
| 100,000 | 1.448 | 69.07M | 1.450 | 68.96M | 0.959 | 0.66× | 0.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.057 | 0.103 | 1.79× |
| 1 | 5 | 0.326 | 0.456 | 1.40× |
| 1 | 10 | 0.395 | 0.933 | 2.36× |
| 10 | 1 | 0.048 | 0.090 | 1.87× |
| 10 | 5 | 0.176 | 0.422 | 2.40× |
| 10 | 10 | 0.381 | 0.903 | 2.37× |
| 100 | 1 | 0.054 | 0.097 | 1.80× |
| 100 | 5 | 0.185 | 0.445 | 2.40× |
| 100 | 10 | 0.386 | 0.901 | 2.33× |
| 1,000 | 1 | 0.056 | 0.094 | 1.66× |
| 1,000 | 5 | 0.197 | 0.511 | 2.59× |
| 1,000 | 10 | 0.426 | 1.012 | 2.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
