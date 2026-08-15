# Donchian benchmark (`Donchian` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 129.56M | 0.006 | 173.92M | 0.571 | 73.96× | 99.28× |
| 10,000 | 0.054 | 185.53M | 0.047 | 213.84M | 4.268 | 79.18× | 91.26× |
| 100,000 | 0.587 | 170.34M | 0.459 | 217.96M | 48.335 | 82.33× | 105.35× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.287 | 3.76× |
| 1 | 5 | 0.270 | 1.113 | 4.12× |
| 1 | 10 | 0.460 | 2.589 | 5.63× |
| 10 | 1 | 0.045 | 0.232 | 5.13× |
| 10 | 5 | 0.202 | 1.465 | 7.26× |
| 10 | 10 | 0.411 | 2.602 | 6.32× |
| 100 | 1 | 0.044 | 0.276 | 6.22× |
| 100 | 5 | 0.214 | 1.668 | 7.81× |
| 100 | 10 | 0.444 | 3.016 | 6.78× |
| 1,000 | 1 | 0.053 | 1.082 | 20.60× |
| 1,000 | 5 | 0.243 | 3.998 | 16.44× |
| 1,000 | 10 | 0.512 | 7.982 | 15.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
