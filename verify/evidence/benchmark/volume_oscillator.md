# VolumeOscillator benchmark (`VolumeOscillator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.084 | 11.85M | 0.075 | 13.30M | 0.188 | 2.23× | 2.50× |
| 10,000 | 0.724 | 13.82M | 0.653 | 15.30M | 0.514 | 0.71× | 0.79× |
| 100,000 | 6.563 | 15.24M | 6.685 | 14.96M | 3.794 | 0.58× | 0.57× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.167 | 0.323 | 1.93× |
| 1 | 5 | 0.444 | 1.366 | 3.08× |
| 1 | 10 | 0.639 | 2.647 | 4.14× |
| 10 | 1 | 0.074 | 0.239 | 3.25× |
| 10 | 5 | 0.312 | 1.356 | 4.35× |
| 10 | 10 | 0.602 | 2.421 | 4.02× |
| 100 | 1 | 0.076 | 0.244 | 3.21× |
| 100 | 5 | 0.312 | 1.435 | 4.60× |
| 100 | 10 | 0.663 | 2.610 | 3.94× |
| 1,000 | 1 | 0.146 | 0.278 | 1.90× |
| 1,000 | 5 | 0.300 | 1.595 | 5.32× |
| 1,000 | 10 | 0.661 | 2.914 | 4.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
