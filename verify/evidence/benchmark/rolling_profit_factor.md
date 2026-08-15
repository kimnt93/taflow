# RollingProfitFactor benchmark (`ProfitFactor` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.75M | 0.022 | 46.29M | 0.170 | 7.77× | 7.86× |
| 10,000 | 0.199 | 50.30M | 0.193 | 51.90M | 0.629 | 3.17× | 3.27× |
| 100,000 | 1.944 | 51.45M | 1.997 | 50.07M | 5.174 | 2.66× | 2.59× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.220 | 2.12× |
| 1 | 5 | 0.245 | 1.007 | 4.11× |
| 1 | 10 | 0.375 | 2.134 | 5.68× |
| 10 | 1 | 0.048 | 0.189 | 3.98× |
| 10 | 5 | 0.183 | 0.943 | 5.16× |
| 10 | 10 | 0.422 | 2.169 | 5.15× |
| 100 | 1 | 0.058 | 0.192 | 3.29× |
| 100 | 5 | 0.199 | 0.961 | 4.83× |
| 100 | 10 | 0.480 | 2.175 | 4.53× |
| 1,000 | 1 | 0.065 | 0.251 | 3.86× |
| 1,000 | 5 | 0.198 | 1.234 | 6.25× |
| 1,000 | 10 | 0.461 | 2.655 | 5.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
