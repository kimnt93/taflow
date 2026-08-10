# CandleClosingMarubozu benchmark (`CDLCLOSINGMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 41.69M | 0.019 | 52.17M | 0.037 | 1.55× | 1.93× |
| 10,000 | 0.147 | 67.87M | 0.141 | 70.74M | 0.128 | 0.87× | 0.91× |
| 100,000 | 1.439 | 69.51M | 1.410 | 70.91M | 1.071 | 0.74× | 0.76× |
| 1,000,000 | 14.951 | 66.88M | 15.874 | 63.00M | 10.658 | 0.71× | 0.67× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.122 | 1.09× |
| 1 | 5 | 0.270 | 0.518 | 1.92× |
| 1 | 10 | 0.555 | 1.027 | 1.85× |
| 10 | 1 | 0.080 | 0.104 | 1.31× |
| 10 | 5 | 0.269 | 0.438 | 1.63× |
| 10 | 10 | 0.515 | 0.903 | 1.75× |
| 100 | 1 | 0.065 | 0.098 | 1.50× |
| 100 | 5 | 0.292 | 0.470 | 1.61× |
| 100 | 10 | 0.622 | 0.921 | 1.48× |
| 1,000 | 1 | 0.082 | 0.108 | 1.32× |
| 1,000 | 5 | 0.272 | 0.506 | 1.86× |
| 1,000 | 10 | 0.582 | 1.041 | 1.79× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
