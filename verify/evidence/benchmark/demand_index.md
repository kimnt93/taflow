# DemandIndex benchmark (`DemandIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 75.52M | 0.010 | 97.16M | 0.154 | 11.64× | 14.98× |
| 10,000 | 0.060 | 166.00M | 0.055 | 182.26M | 1.274 | 21.15× | 23.22× |
| 100,000 | 0.513 | 194.76M | 0.539 | 185.66M | 12.680 | 24.70× | 23.54× |
| 1,000,000 | 5.825 | 171.67M | 5.225 | 191.38M | 141.307 | 24.26× | 27.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | 0.112 | 1.11× |
| 1 | 5 | 0.295 | 0.430 | 1.46× |
| 1 | 10 | 0.582 | 0.816 | 1.40× |
| 10 | 1 | 0.063 | 0.080 | 1.28× |
| 10 | 5 | 0.278 | 0.500 | 1.80× |
| 10 | 10 | 0.636 | 0.850 | 1.34× |
| 100 | 1 | 0.062 | 0.111 | 1.79× |
| 100 | 5 | 0.326 | 0.507 | 1.56× |
| 100 | 10 | 0.767 | 1.125 | 1.47× |
| 1,000 | 1 | 0.062 | 0.209 | 3.39× |
| 1,000 | 5 | 0.301 | 1.014 | 3.37× |
| 1,000 | 10 | 0.556 | 2.203 | 3.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
