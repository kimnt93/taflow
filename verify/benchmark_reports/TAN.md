# MathTan benchmark (`TAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 53.06M | 0.020 | 50.52M | 0.046 | 2.46× | 2.34× |
| 10,000 | 0.211 | 47.35M | 0.224 | 44.55M | 0.241 | 1.14× | 1.07× |
| 100,000 | 2.329 | 42.94M | 2.193 | 45.60M | 2.080 | 0.89× | 0.95× |
| 1,000,000 | 21.774 | 45.93M | 22.166 | 45.12M | 20.856 | 0.96× | 0.94× |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.128 ms**; native kernel **2.081 ms**; TA-Lib 2.085 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.281 | 0.177 | 5.65M | 2020.574 | 11407.44× | 137.89× |
| 100,000 | 10 | 1.142 | 0.734 | 13.62M | 2007.281 | 2733.31× | 33.25× |
| 100,000 | 1,000 | 25.442 | 22.014 | 45.43M | 2008.832 | 91.25× | 1.92× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 44.77M | 46.64M | 1.00× | 2.54M | 2.72M | 1.00× | 42.84M |
| 2 | 79.63M | 71.81M | 1.54× | 2.82M | 2.86M | 1.05× | 45.83M |
| 4 | 131.92M | 116.47M | 2.50× | 2.50M | 2.48M | 0.91× | 44.77M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
