# TrueStrengthIndex benchmark (`TSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 98.16M | 0.009 | 114.91M | 0.223 | 21.93× | 25.67× |
| 10,000 | 0.070 | 143.03M | 0.068 | 148.10M | 0.589 | 8.43× | 8.73× |
| 100,000 | 0.664 | 150.58M | 0.638 | 156.69M | 4.328 | 6.52× | 6.78× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | 0.267 | 3.38× |
| 1 | 5 | 0.263 | 1.367 | 5.20× |
| 1 | 10 | 0.402 | 2.766 | 6.87× |
| 10 | 1 | 0.049 | 0.242 | 4.94× |
| 10 | 5 | 0.189 | 1.366 | 7.22× |
| 10 | 10 | 0.399 | 2.508 | 6.29× |
| 100 | 1 | 0.046 | 0.251 | 5.44× |
| 100 | 5 | 0.219 | 1.442 | 6.59× |
| 100 | 10 | 0.430 | 2.652 | 6.17× |
| 1,000 | 1 | 0.053 | 0.284 | 5.30× |
| 1,000 | 5 | 0.201 | 1.578 | 7.85× |
| 1,000 | 10 | 0.426 | 2.891 | 6.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
