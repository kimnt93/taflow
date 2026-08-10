# MathLn benchmark (`LN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 110.18M | 0.008 | 125.26M | 0.037 | 4.12× | 4.68× |
| 10,000 | 0.060 | 167.91M | 0.055 | 182.46M | 0.080 | 1.35× | 1.46× |
| 100,000 | 0.552 | 181.32M | 0.605 | 165.41M | 0.517 | 0.94× | 0.86× |
| 1,000,000 | 5.542 | 180.45M | 5.375 | 186.04M | 5.113 | 0.92× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 0.123 | 1.17× |
| 1 | 5 | 0.300 | 0.486 | 1.62× |
| 1 | 10 | 0.555 | 1.085 | 1.95× |
| 10 | 1 | 0.047 | 0.090 | 1.90× |
| 10 | 5 | 0.230 | 0.482 | 2.09× |
| 10 | 10 | 0.559 | 1.570 | 2.81× |
| 100 | 1 | 0.057 | 0.086 | 1.51× |
| 100 | 5 | 0.248 | 0.430 | 1.74× |
| 100 | 10 | 0.528 | 1.093 | 2.07× |
| 1,000 | 1 | 0.067 | 0.111 | 1.66× |
| 1,000 | 5 | 0.252 | 0.486 | 1.93× |
| 1,000 | 10 | 0.545 | 1.166 | 2.14× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
