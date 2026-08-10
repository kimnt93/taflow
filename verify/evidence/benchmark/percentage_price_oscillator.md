# PercentagePriceOscillator benchmark (`PPO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 131.20M | 0.007 | 146.25M | 0.040 | 5.29× | 5.89× |
| 10,000 | 0.045 | 222.18M | 0.042 | 238.72M | 0.079 | 1.75× | 1.88× |
| 100,000 | 0.400 | 250.16M | 0.373 | 267.83M | 0.478 | 1.19× | 1.28× |
| 1,000,000 | 4.194 | 238.44M | 3.929 | 254.54M | 4.935 | 1.18× | 1.26× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.111 | 1.11× |
| 1 | 5 | 0.358 | 0.485 | 1.35× |
| 1 | 10 | 0.506 | 0.988 | 1.95× |
| 10 | 1 | 0.049 | 0.100 | 2.04× |
| 10 | 5 | 0.222 | 0.485 | 2.19× |
| 10 | 10 | 0.478 | 0.979 | 2.05× |
| 100 | 1 | 0.046 | 0.093 | 2.01× |
| 100 | 5 | 0.234 | 0.468 | 2.01× |
| 100 | 10 | 0.516 | 0.994 | 1.93× |
| 1,000 | 1 | 0.055 | 0.114 | 2.07× |
| 1,000 | 5 | 0.257 | 0.504 | 1.97× |
| 1,000 | 10 | 0.520 | 1.090 | 2.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
