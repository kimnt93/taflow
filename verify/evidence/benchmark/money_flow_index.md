# MoneyFlowIndex benchmark (`MFI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.090 | 11.14M | 0.083 | 12.01M | 0.037 | 0.41× | 0.44× |
| 10,000 | 0.734 | 13.62M | 0.707 | 14.15M | 0.105 | 0.14× | 0.15× |
| 100,000 | 7.053 | 14.18M | 6.976 | 14.34M | 0.851 | 0.12× | 0.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.113 | 1.03× |
| 1 | 5 | 0.459 | 0.501 | 1.09× |
| 1 | 10 | 0.653 | 0.973 | 1.49× |
| 10 | 1 | 0.074 | 0.095 | 1.28× |
| 10 | 5 | 0.320 | 0.455 | 1.42× |
| 10 | 10 | 0.654 | 0.995 | 1.52× |
| 100 | 1 | 0.082 | 0.095 | 1.16× |
| 100 | 5 | 0.331 | 0.475 | 1.44× |
| 100 | 10 | 0.672 | 0.984 | 1.46× |
| 1,000 | 1 | 0.156 | 0.112 | 0.72× |
| 1,000 | 5 | 0.346 | 0.519 | 1.50× |
| 1,000 | 10 | 0.705 | 1.046 | 1.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
