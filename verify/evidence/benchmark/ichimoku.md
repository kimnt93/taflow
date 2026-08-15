# Ichimoku benchmark (`causal ichimoku components` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.087 | 11.43M | 0.086 | 11.64M | 0.427 | 4.89× | 4.98× |
| 10,000 | 0.888 | 11.26M | 0.930 | 10.76M | 2.478 | 2.79× | 2.67× |
| 100,000 | 8.704 | 11.49M | 8.816 | 11.34M | 25.053 | 2.88× | 2.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.170 | 1.53× |
| 1 | 5 | 0.297 | 0.646 | 2.18× |
| 1 | 10 | 0.417 | 1.235 | 2.96× |
| 10 | 1 | 0.050 | 0.212 | 4.27× |
| 10 | 5 | 0.210 | 0.997 | 4.74× |
| 10 | 10 | 0.422 | 1.980 | 4.69× |
| 100 | 1 | 0.054 | 0.326 | 6.06× |
| 100 | 5 | 0.207 | 1.764 | 8.50× |
| 100 | 10 | 0.447 | 3.582 | 8.01× |
| 1,000 | 1 | 0.142 | 0.551 | 3.89× |
| 1,000 | 5 | 0.257 | 2.093 | 8.15× |
| 1,000 | 10 | 0.523 | 4.341 | 8.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
