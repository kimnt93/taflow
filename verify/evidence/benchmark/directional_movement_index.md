# DirectionalMovementIndex benchmark (`DX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.54M | 0.017 | 60.22M | 0.042 | 2.35× | 2.50× |
| 10,000 | 0.108 | 92.47M | 0.104 | 96.01M | 0.127 | 1.18× | 1.22× |
| 100,000 | 1.028 | 97.25M | 0.966 | 103.55M | 0.871 | 0.85× | 0.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.121 | 0.106 | 0.87× |
| 1 | 5 | 0.336 | 0.522 | 1.56× |
| 1 | 10 | 0.522 | 0.942 | 1.81× |
| 10 | 1 | 0.053 | 0.090 | 1.70× |
| 10 | 5 | 0.236 | 0.467 | 1.98× |
| 10 | 10 | 0.538 | 0.971 | 1.81× |
| 100 | 1 | 0.057 | 0.089 | 1.57× |
| 100 | 5 | 0.266 | 0.442 | 1.66× |
| 100 | 10 | 0.533 | 1.014 | 1.90× |
| 1,000 | 1 | 0.068 | 0.099 | 1.46× |
| 1,000 | 5 | 0.291 | 0.525 | 1.81× |
| 1,000 | 10 | 0.578 | 1.140 | 1.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
