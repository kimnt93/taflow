# EvenBetterSinewave benchmark (`ebsw` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 111.48M | 0.008 | 122.78M | 11.991 | 1336.71× | 1472.22× |
| 10,000 | 0.063 | 158.04M | 0.062 | 160.12M | 125.869 | 1989.23× | 2015.48× |
| 100,000 | 0.617 | 162.02M | 0.598 | 167.15M | 1222.723 | 1981.07× | 2043.72× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.199 | 2.27× |
| 1 | 5 | 0.245 | 0.751 | 3.06× |
| 1 | 10 | 0.469 | 1.531 | 3.26× |
| 10 | 1 | 0.050 | 0.149 | 2.97× |
| 10 | 5 | 0.219 | 0.743 | 3.39× |
| 10 | 10 | 0.466 | 1.469 | 3.15× |
| 100 | 1 | 0.049 | 0.984 | 20.25× |
| 100 | 5 | 0.246 | 4.992 | 20.30× |
| 100 | 10 | 0.474 | 10.471 | 22.08× |
| 1,000 | 1 | 0.056 | 11.577 | 207.80× |
| 1,000 | 5 | 0.442 | 58.737 | 133.04× |
| 1,000 | 10 | 0.527 | 178.812 | 339.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
