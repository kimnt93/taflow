# AroonOscillator benchmark (`AROONOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 78.87M | 0.011 | 88.61M | 0.039 | 3.09× | 3.47× |
| 10,000 | 0.129 | 77.80M | 0.125 | 79.88M | 0.141 | 1.09× | 1.12× |
| 100,000 | 1.312 | 76.23M | 1.180 | 84.75M | 1.060 | 0.81× | 0.90× |
| 1,000,000 | 12.571 | 79.55M | 12.538 | 79.76M | 10.763 | 0.86× | 0.86× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.117 | 1.67× |
| 1 | 5 | 0.357 | 0.556 | 1.56× |
| 1 | 10 | 0.506 | 0.946 | 1.87× |
| 10 | 1 | 0.051 | 0.090 | 1.78× |
| 10 | 5 | 0.257 | 0.498 | 1.94× |
| 10 | 10 | 0.506 | 1.025 | 2.02× |
| 100 | 1 | 0.053 | 0.095 | 1.80× |
| 100 | 5 | 0.260 | 0.469 | 1.80× |
| 100 | 10 | 0.522 | 1.018 | 1.95× |
| 1,000 | 1 | 0.066 | 0.107 | 1.63× |
| 1,000 | 5 | 0.241 | 0.512 | 2.12× |
| 1,000 | 10 | 0.561 | 1.151 | 2.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
