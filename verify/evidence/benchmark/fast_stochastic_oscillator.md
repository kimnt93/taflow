# FastStochasticOscillator benchmark (`STOCHF` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 64.52M | 0.014 | 70.38M | 0.049 | 3.16× | 3.45× |
| 10,000 | 0.129 | 77.25M | 0.119 | 83.82M | 0.150 | 1.16× | 1.26× |
| 100,000 | 1.276 | 78.38M | 1.272 | 78.60M | 1.045 | 0.82× | 0.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.161 | 2.02× |
| 1 | 5 | 0.215 | 0.556 | 2.59× |
| 1 | 10 | 0.442 | 1.038 | 2.35× |
| 10 | 1 | 0.046 | 0.100 | 2.18× |
| 10 | 5 | 0.199 | 0.488 | 2.46× |
| 10 | 10 | 0.409 | 1.145 | 2.80× |
| 100 | 1 | 0.049 | 0.101 | 2.07× |
| 100 | 5 | 0.218 | 0.592 | 2.71× |
| 100 | 10 | 0.475 | 1.151 | 2.42× |
| 1,000 | 1 | 0.072 | 0.117 | 1.62× |
| 1,000 | 5 | 0.226 | 0.615 | 2.72× |
| 1,000 | 10 | 0.466 | 1.235 | 2.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
