# AwesomeOscillator benchmark (`AwesomeOscillator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 34.61M | 0.027 | 37.21M | 0.250 | 8.64× | 9.29× |
| 10,000 | 0.260 | 38.48M | 0.247 | 40.55M | 0.865 | 3.33× | 3.51× |
| 100,000 | 2.445 | 40.90M | 2.345 | 42.65M | 7.379 | 3.02× | 3.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.308 | 3.93× |
| 1 | 5 | 0.303 | 1.407 | 4.64× |
| 1 | 10 | 0.516 | 2.949 | 5.71× |
| 10 | 1 | 0.057 | 0.251 | 4.38× |
| 10 | 5 | 0.249 | 1.514 | 6.09× |
| 10 | 10 | 0.503 | 2.588 | 5.14× |
| 100 | 1 | 0.056 | 0.260 | 4.67× |
| 100 | 5 | 0.285 | 1.479 | 5.18× |
| 100 | 10 | 0.492 | 2.797 | 5.69× |
| 1,000 | 1 | 0.078 | 0.323 | 4.15× |
| 1,000 | 5 | 0.248 | 1.776 | 7.15× |
| 1,000 | 10 | 0.545 | 3.424 | 6.28× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
