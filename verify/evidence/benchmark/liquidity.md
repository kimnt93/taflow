# Liquidity benchmark (`causal liquidity pools` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.039 | 25.42M | 0.038 | 26.45M | 4.664 | 118.59× | 123.39× |
| 10,000 | 0.418 | 23.93M | 0.405 | 24.69M | 68.982 | 165.11× | 170.30× |
| 100,000 | 4.508 | 22.18M | 4.463 | 22.40M | 1169.804 | 259.51× | 262.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.168 | 0.184 | 1.10× |
| 1 | 5 | 0.349 | 0.618 | 1.77× |
| 1 | 10 | 0.480 | 1.172 | 2.44× |
| 10 | 1 | 0.056 | 0.127 | 2.30× |
| 10 | 5 | 0.257 | 0.631 | 2.46× |
| 10 | 10 | 0.544 | 1.253 | 2.30× |
| 100 | 1 | 0.056 | 0.222 | 3.98× |
| 100 | 5 | 0.252 | 1.033 | 4.10× |
| 100 | 10 | 0.559 | 2.073 | 3.71× |
| 1,000 | 1 | 0.096 | 4.733 | 49.06× |
| 1,000 | 5 | 0.298 | 24.703 | 82.79× |
| 1,000 | 10 | 0.657 | 48.872 | 74.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
