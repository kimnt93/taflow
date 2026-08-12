# UltimateOscillator benchmark (`ULTOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.10M | 0.018 | 55.94M | 0.059 | 2.95× | 3.29× |
| 10,000 | 0.149 | 67.18M | 0.132 | 76.01M | 0.186 | 1.25× | 1.41× |
| 100,000 | 1.327 | 75.38M | 1.282 | 78.00M | 1.563 | 1.18× | 1.22× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.116 | 1.22× |
| 1 | 5 | 0.289 | 0.521 | 1.80× |
| 1 | 10 | 0.485 | 0.964 | 1.99× |
| 10 | 1 | 0.049 | 0.100 | 2.06× |
| 10 | 5 | 0.239 | 0.486 | 2.03× |
| 10 | 10 | 0.534 | 1.027 | 1.92× |
| 100 | 1 | 0.057 | 0.100 | 1.74× |
| 100 | 5 | 0.247 | 0.471 | 1.91× |
| 100 | 10 | 0.519 | 1.010 | 1.95× |
| 1,000 | 1 | 0.069 | 0.123 | 1.77× |
| 1,000 | 5 | 0.298 | 0.555 | 1.86× |
| 1,000 | 10 | 0.547 | 1.171 | 2.14× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
