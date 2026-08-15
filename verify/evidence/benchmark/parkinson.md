# Parkinson benchmark (`ParkinsonVolatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 61.50M | 0.015 | 68.85M | 0.222 | 13.63× | 15.26× |
| 10,000 | 0.136 | 73.58M | 0.133 | 75.18M | 0.877 | 6.45× | 6.60× |
| 100,000 | 1.366 | 73.22M | 1.331 | 75.15M | 7.787 | 5.70× | 5.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.271 | 2.97× |
| 1 | 5 | 0.211 | 1.345 | 6.39× |
| 1 | 10 | 0.390 | 2.659 | 6.82× |
| 10 | 1 | 0.048 | 0.239 | 4.98× |
| 10 | 5 | 0.191 | 1.416 | 7.41× |
| 10 | 10 | 0.426 | 2.564 | 6.02× |
| 100 | 1 | 0.048 | 0.243 | 5.07× |
| 100 | 5 | 0.209 | 1.493 | 7.15× |
| 100 | 10 | 0.421 | 2.912 | 6.91× |
| 1,000 | 1 | 0.078 | 0.340 | 4.35× |
| 1,000 | 5 | 0.200 | 1.800 | 8.99× |
| 1,000 | 10 | 0.424 | 3.335 | 7.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
