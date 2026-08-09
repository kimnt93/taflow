# Momentum benchmark (`MOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 169.11M | 0.005 | 214.33M | 0.037 | 6.33× | 8.02× |
| 10,000 | 0.027 | 373.76M | 0.023 | 429.98M | 0.039 | 1.44× | 1.66× |
| 100,000 | 0.223 | 447.60M | 0.197 | 508.62M | 0.073 | 0.33× | 0.37× |
| 1,000,000 | 2.786 | 358.92M | 2.126 | 470.32M | 0.707 | 0.25× | 0.33× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.112 | 1.08× |
| 1 | 5 | 0.272 | 0.507 | 1.86× |
| 1 | 10 | 0.550 | 1.078 | 1.96× |
| 10 | 1 | 0.064 | 0.112 | 1.75× |
| 10 | 5 | 0.234 | 0.477 | 2.04× |
| 10 | 10 | 0.513 | 0.995 | 1.94× |
| 100 | 1 | 0.055 | 0.099 | 1.79× |
| 100 | 5 | 0.235 | 0.443 | 1.89× |
| 100 | 10 | 0.501 | 0.966 | 1.93× |
| 1,000 | 1 | 0.058 | 0.101 | 1.73× |
| 1,000 | 5 | 0.247 | 0.490 | 1.98× |
| 1,000 | 10 | 0.513 | 0.955 | 1.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
