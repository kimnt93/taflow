# AccumulationDistributionOscillator benchmark (`ADOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.063 | 15.78M | 0.057 | 17.57M | 0.037 | 0.59× | 0.66× |
| 10,000 | 0.702 | 14.25M | 0.461 | 21.70M | 0.056 | 0.08× | 0.12× |
| 100,000 | 4.650 | 21.51M | 4.585 | 21.81M | 0.287 | 0.06× | 0.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.124 | 0.133 | 1.07× |
| 1 | 5 | 0.473 | 0.518 | 1.10× |
| 1 | 10 | 0.668 | 0.961 | 1.44× |
| 10 | 1 | 0.127 | 0.098 | 0.77× |
| 10 | 5 | 0.331 | 0.466 | 1.41× |
| 10 | 10 | 0.660 | 0.952 | 1.44× |
| 100 | 1 | 0.075 | 0.090 | 1.21× |
| 100 | 5 | 0.305 | 0.449 | 1.47× |
| 100 | 10 | 0.668 | 0.958 | 1.43× |
| 1,000 | 1 | 0.114 | 0.105 | 0.92× |
| 1,000 | 5 | 0.312 | 0.464 | 1.49× |
| 1,000 | 10 | 0.680 | 0.980 | 1.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
