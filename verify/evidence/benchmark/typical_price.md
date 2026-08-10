# TypicalPrice benchmark (`TYPPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 135.38M | 0.006 | 174.74M | 0.032 | 4.34× | 5.60× |
| 10,000 | 0.025 | 396.26M | 0.021 | 469.53M | 0.039 | 1.54× | 1.83× |
| 100,000 | 0.211 | 473.68M | 0.170 | 589.33M | 0.098 | 0.47× | 0.58× |
| 1,000,000 | 2.883 | 346.89M | 2.354 | 424.88M | 1.703 | 0.59× | 0.72× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.159 | 1.88× |
| 1 | 5 | 0.267 | 0.536 | 2.01× |
| 1 | 10 | 0.559 | 0.965 | 1.73× |
| 10 | 1 | 0.065 | 0.122 | 1.88× |
| 10 | 5 | 0.292 | 0.506 | 1.73× |
| 10 | 10 | 0.503 | 0.969 | 1.92× |
| 100 | 1 | 0.058 | 0.108 | 1.85× |
| 100 | 5 | 0.283 | 0.521 | 1.84× |
| 100 | 10 | 0.573 | 0.959 | 1.67× |
| 1,000 | 1 | 0.062 | 0.092 | 1.49× |
| 1,000 | 5 | 0.261 | 0.461 | 1.77× |
| 1,000 | 10 | 0.545 | 0.922 | 1.69× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
