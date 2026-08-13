# ChandeMomentumOscillator benchmark (`CMO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.039 | 25.81M | 0.033 | 30.26M | 0.037 | 0.95× | 1.11× |
| 10,000 | 0.284 | 35.22M | 0.272 | 36.78M | 0.086 | 0.30× | 0.32× |
| 100,000 | 2.716 | 36.82M | 2.661 | 37.58M | 0.572 | 0.21× | 0.22× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.162 | 0.182 | 1.12× |
| 1 | 5 | 0.472 | 0.533 | 1.13× |
| 1 | 10 | 0.633 | 0.969 | 1.53× |
| 10 | 1 | 0.060 | 0.090 | 1.50× |
| 10 | 5 | 0.295 | 0.444 | 1.50× |
| 10 | 10 | 0.590 | 0.979 | 1.66× |
| 100 | 1 | 0.062 | 0.097 | 1.56× |
| 100 | 5 | 0.282 | 0.443 | 1.57× |
| 100 | 10 | 0.592 | 0.935 | 1.58× |
| 1,000 | 1 | 0.095 | 0.096 | 1.00× |
| 1,000 | 5 | 0.281 | 0.473 | 1.68× |
| 1,000 | 10 | 0.609 | 1.003 | 1.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
