# ChandeMomentumOscillator benchmark (`CMO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 110.54M | 0.008 | 128.69M | 0.042 | 4.60× | 5.35× |
| 10,000 | 0.063 | 159.62M | 0.057 | 175.47M | 0.102 | 1.62× | 1.78× |
| 100,000 | 0.570 | 175.45M | 0.556 | 180.02M | 0.636 | 1.12× | 1.14× |
| 1,000,000 | 5.936 | 168.47M | 5.488 | 182.22M | 5.991 | 1.01× | 1.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.125 | 1.43× |
| 1 | 5 | 0.276 | 0.504 | 1.83× |
| 1 | 10 | 0.477 | 1.043 | 2.19× |
| 10 | 1 | 0.067 | 0.104 | 1.54× |
| 10 | 5 | 0.248 | 0.503 | 2.03× |
| 10 | 10 | 0.512 | 1.070 | 2.09× |
| 100 | 1 | 0.058 | 0.099 | 1.72× |
| 100 | 5 | 0.262 | 0.553 | 2.11× |
| 100 | 10 | 0.532 | 1.065 | 2.00× |
| 1,000 | 1 | 0.056 | 0.104 | 1.84× |
| 1,000 | 5 | 0.249 | 0.513 | 2.06× |
| 1,000 | 10 | 0.529 | 1.113 | 2.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
