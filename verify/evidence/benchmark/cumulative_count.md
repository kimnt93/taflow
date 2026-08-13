# CumulativeCount benchmark (`one-based cumulative count` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 46.97M | 0.016 | 62.49M | 0.012 | 0.55× | 0.74× |
| 10,000 | 0.110 | 91.18M | 0.107 | 93.26M | 0.016 | 0.15× | 0.15× |
| 100,000 | 0.978 | 102.24M | 0.970 | 103.07M | 0.058 | 0.06× | 0.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.187 | 0.129 | 0.69× |
| 1 | 5 | 0.445 | 0.280 | 0.63× |
| 1 | 10 | 0.568 | 0.561 | 0.99× |
| 10 | 1 | 0.066 | 0.056 | 0.85× |
| 10 | 5 | 0.280 | 0.278 | 0.99× |
| 10 | 10 | 0.543 | 0.572 | 1.05× |
| 100 | 1 | 0.071 | 0.055 | 0.78× |
| 100 | 5 | 0.283 | 0.264 | 0.93× |
| 100 | 10 | 0.571 | 0.586 | 1.03× |
| 1,000 | 1 | 0.074 | 0.057 | 0.77× |
| 1,000 | 5 | 0.284 | 0.284 | 1.00× |
| 1,000 | 10 | 0.568 | 0.586 | 1.03× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
