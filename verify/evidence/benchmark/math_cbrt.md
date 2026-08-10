# MathCbrt benchmark (`numpy.cbrt` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.56M | 0.019 | 52.80M | 0.026 | 1.30× | 1.38× |
| 10,000 | 0.168 | 59.56M | 0.164 | 60.89M | 0.148 | 0.88× | 0.90× |
| 100,000 | 1.657 | 60.34M | 1.621 | 61.69M | 1.379 | 0.83× | 0.85× |
| 1,000,000 | 17.184 | 58.19M | 16.719 | 59.81M | 13.869 | 0.81× | 0.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.074 | 0.89× |
| 1 | 5 | 0.387 | 0.331 | 0.86× |
| 1 | 10 | 0.476 | 0.576 | 1.21× |
| 10 | 1 | 0.051 | 0.057 | 1.13× |
| 10 | 5 | 0.217 | 0.271 | 1.25× |
| 10 | 10 | 0.461 | 0.559 | 1.21× |
| 100 | 1 | 0.047 | 0.059 | 1.26× |
| 100 | 5 | 0.238 | 0.277 | 1.16× |
| 100 | 10 | 0.479 | 0.576 | 1.20× |
| 1,000 | 1 | 0.065 | 0.072 | 1.12× |
| 1,000 | 5 | 0.221 | 0.312 | 1.41× |
| 1,000 | 10 | 0.510 | 0.756 | 1.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
