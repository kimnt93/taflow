# Liquidity benchmark (`causal liquidity pools` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.041 | 24.60M | 0.038 | 26.58M | 4.760 | 117.07× | 126.54× |
| 10,000 | 0.411 | 24.35M | 0.427 | 23.44M | 65.473 | 159.40× | 153.44× |
| 100,000 | 4.356 | 22.96M | 4.238 | 23.59M | 1100.010 | 252.52× | 259.53× |
| 1,000,000 | 59.468 | 16.82M | 45.754 | 21.86M | 13514.271 | 227.25× | 295.37× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.135 | 1.54× |
| 1 | 5 | 0.296 | 0.593 | 2.00× |
| 1 | 10 | 0.653 | 1.207 | 1.85× |
| 10 | 1 | 0.055 | 0.127 | 2.32× |
| 10 | 5 | 0.237 | 0.638 | 2.70× |
| 10 | 10 | 0.509 | 1.266 | 2.49× |
| 100 | 1 | 0.055 | 0.212 | 3.83× |
| 100 | 5 | 0.260 | 1.012 | 3.89× |
| 100 | 10 | 0.527 | 2.014 | 3.82× |
| 1,000 | 1 | 0.091 | 4.838 | 52.88× |
| 1,000 | 5 | 0.282 | 25.184 | 89.16× |
| 1,000 | 10 | 0.662 | 56.677 | 85.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
