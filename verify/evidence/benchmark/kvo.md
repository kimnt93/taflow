# KlingerVolumeOscillator benchmark (`KVO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.34M | 0.016 | 64.24M | 0.332 | 17.38× | 21.33× |
| 10,000 | 0.156 | 63.99M | 0.153 | 65.56M | 1.559 | 9.98× | 10.22× |
| 100,000 | 1.552 | 64.42M | 1.482 | 67.48M | 14.641 | 9.43× | 9.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.334 | 3.82× |
| 1 | 5 | 0.273 | 1.571 | 5.75× |
| 1 | 10 | 0.420 | 2.707 | 6.45× |
| 10 | 1 | 0.046 | 0.280 | 6.10× |
| 10 | 5 | 0.233 | 1.577 | 6.78× |
| 10 | 10 | 0.441 | 3.036 | 6.88× |
| 100 | 1 | 0.056 | 0.285 | 5.06× |
| 100 | 5 | 0.215 | 1.620 | 7.54× |
| 100 | 10 | 0.477 | 3.025 | 6.35× |
| 1,000 | 1 | 0.073 | 0.390 | 5.32× |
| 1,000 | 5 | 0.210 | 2.322 | 11.08× |
| 1,000 | 10 | 0.468 | 4.436 | 9.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
