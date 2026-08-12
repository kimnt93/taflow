# RollingDrawdownDuration benchmark (`DrawdownDuration` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 170.99M | 0.005 | 203.58M | 0.127 | 21.77× | 25.92× |
| 10,000 | 0.029 | 349.48M | 0.026 | 387.95M | 0.426 | 14.88× | 16.51× |
| 100,000 | 0.225 | 444.33M | 0.209 | 479.55M | 3.442 | 15.29× | 16.51× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.204 | 2.53× |
| 1 | 5 | 0.284 | 0.831 | 2.93× |
| 1 | 10 | 0.479 | 1.817 | 3.80× |
| 10 | 1 | 0.067 | 0.169 | 2.52× |
| 10 | 5 | 0.254 | 0.853 | 3.36× |
| 10 | 10 | 0.500 | 1.690 | 3.38× |
| 100 | 1 | 0.049 | 0.171 | 3.50× |
| 100 | 5 | 0.280 | 1.171 | 4.18× |
| 100 | 10 | 0.496 | 1.667 | 3.36× |
| 1,000 | 1 | 0.069 | 0.221 | 3.21× |
| 1,000 | 5 | 0.265 | 1.303 | 4.91× |
| 1,000 | 10 | 0.519 | 2.065 | 3.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
