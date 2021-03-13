import sys

# from scipy.special import comb


end = int(sys.argv[1])

comb_cache = [1]


def comb(n, k):
    result = comb_cache[k - 1] * n // k
    if k == len(comb_cache):
        comb_cache.append(result)
    else:
        comb_cache[k] = result
    return result


fub = [1]
for n in range(1, end + 1):
    fub.append(sum(comb(n, i) * fub[n - i] for i in range(n, 0, -1)))

print(fub)
