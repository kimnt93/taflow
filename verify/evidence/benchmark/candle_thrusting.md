# CandleThrusting benchmark (`CDLTHRUSTING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.91M | 0.015 | 67.83M | 0.039 | 2.20× | 2.62× |
| 10,000 | 0.146 | 68.30M | 0.141 | 70.93M | 0.129 | 0.88× | 0.91× |
| 100,000 | 1.438 | 69.56M | 1.436 | 69.63M | 0.953 | 0.66× | 0.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.140 | 0.148 | 1.06× |
| 1 | 5 | 0.356 | 0.452 | 1.27× |
| 1 | 10 | 0.525 | 0.908 | 1.73× |
| 10 | 1 | 0.062 | 0.093 | 1.50× |
| 10 | 5 | 0.278 | 0.478 | 1.72× |
| 10 | 10 | 0.536 | 0.899 | 1.68× |
| 100 | 1 | 0.059 | 0.086 | 1.46× |
| 100 | 5 | 0.254 | 0.426 | 1.68× |
| 100 | 10 | 0.615 | 0.942 | 1.53× |
| 1,000 | 1 | 0.067 | 0.104 | 1.56× |
| 1,000 | 5 | 0.260 | 0.471 | 1.81× |
| 1,000 | 10 | 0.570 | 1.090 | 1.91× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
