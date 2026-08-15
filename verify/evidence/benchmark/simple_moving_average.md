# SimpleMovingAverage benchmark (`SMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 257.66M | 0.003 | 323.70M | 0.034 | 8.71× | 10.94× |
| 10,000 | 0.024 | 423.74M | 0.021 | 480.61M | 0.051 | 2.17× | 2.46× |
| 100,000 | 0.227 | 440.71M | 0.196 | 509.60M | 0.222 | 0.98× | 1.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.062 | 0.112 | 1.80× |
| 1 | 5 | 0.227 | 0.479 | 2.11× |
| 1 | 10 | 0.403 | 1.043 | 2.59× |
| 10 | 1 | 0.046 | 0.109 | 2.35× |
| 10 | 5 | 0.196 | 0.461 | 2.35× |
| 10 | 10 | 0.416 | 0.969 | 2.33× |
| 100 | 1 | 0.047 | 0.102 | 2.19× |
| 100 | 5 | 0.198 | 0.523 | 2.64× |
| 100 | 10 | 0.447 | 0.989 | 2.22× |
| 1,000 | 1 | 0.044 | 0.095 | 2.13× |
| 1,000 | 5 | 0.215 | 0.487 | 2.26× |
| 1,000 | 10 | 0.471 | 1.021 | 2.17× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
