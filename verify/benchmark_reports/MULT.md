# MathMultiply benchmark (`MULT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 410.97M | 0.001 | 943.96M | 0.029 | 11.85× | 27.22× |
| 10,000 | 0.007 | 1.36G | 0.004 | 2.39G | 0.035 | 4.73× | 8.32× |
| 100,000 | 0.064 | 1.56G | 0.040 | 2.53G | 0.068 | 1.06× | 1.72× |
| 1,000,000 | 1.158 | 863.65M | 0.798 | 1.25G | 0.829 | 0.72× | 1.04× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.065 ms**; native kernel **0.039 ms**; TA-Lib 0.068 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.252 | 0.175 | 5.73M | 68.838 | 394.33× | 158.57× |
| 100,000 | 10 | 1.340 | 0.651 | 15.37M | 69.184 | 106.33× | 43.56× |
| 100,000 | 1,000 | 3.810 | 2.070 | 483.03M | 69.159 | 33.41× | 14.02× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 563.25M | 916.37M | 1.00× | 3.03M | 3.34M | 1.00× | 633.18M |
| 2 | 930.02M | 1.68G | 1.83× | 2.57M | 3.62M | 1.09× | 684.26M |
| 4 | 995.86M | 2.13G | 2.33× | 2.75M | 3.16M | 0.95× | 648.71M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
