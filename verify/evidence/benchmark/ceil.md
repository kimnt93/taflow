# MathCeil benchmark (`CEIL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 185.76M | 0.005 | 222.19M | 0.031 | 5.69× | 6.80× |
| 10,000 | 0.028 | 362.89M | 0.025 | 397.29M | 0.042 | 1.52× | 1.66× |
| 100,000 | 0.248 | 402.57M | 0.225 | 445.37M | 0.166 | 0.67× | 0.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.107 | 1.22× |
| 1 | 5 | 0.312 | 0.494 | 1.58× |
| 1 | 10 | 0.494 | 0.895 | 1.81× |
| 10 | 1 | 0.047 | 0.086 | 1.84× |
| 10 | 5 | 0.224 | 0.430 | 1.92× |
| 10 | 10 | 0.490 | 0.944 | 1.93× |
| 100 | 1 | 0.052 | 0.093 | 1.79× |
| 100 | 5 | 0.231 | 0.410 | 1.78× |
| 100 | 10 | 0.459 | 0.930 | 2.03× |
| 1,000 | 1 | 0.065 | 0.100 | 1.54× |
| 1,000 | 5 | 0.245 | 0.434 | 1.77× |
| 1,000 | 10 | 0.492 | 0.964 | 1.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
