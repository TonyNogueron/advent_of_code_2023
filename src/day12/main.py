file_path = "p1.txt"

# use a dictionary to store calculations
dp = {}

with open(file_path, 'r') as file:
    lines = file.readlines()
    lines = [line.strip() for line in lines]


def count(condition, valid):
    if len(condition) == 0:  # If we got to the end and there are no more blocks to fill we are done
        # and this is a valid solution
        return 1 if valid == () else 0

    if valid == ():  # If we do not have blocks to fill, but there are blocks left, this is not valid
        return 0 if '#' in condition else 1

    key = (condition, valid)
    if key in dp:
        return dp[key]

    ans = 0
    if condition[0] == '.' or condition[0] == '?':
        # This is the case where we skip a dot, or we decide to replace the '?' with a dot,
        # so we need to continue without changing the valid part and just move on
        ans += count(condition[1:], valid)

    if condition[0] == '#' or condition[0] == '?':
        # This is the case where we find a block or decide to replace the '?' with a block
        if (valid[0] <= len(condition)  # If we need more blocks than we have left is not valid
                and '.' not in condition[0:valid[0]]  # If there is a dot in the block it is not valid
                and (valid[0] == len(condition)  # If we are at the final block we can continue
                     or condition[valid[0]] != '#')):  # if we can fill the block and end it, then it is valid
            ans += count(condition[valid[0] + 1:], valid[1:])  # We filled the block and remove it

    dp[key] = ans
    return ans


result = 0
for line in lines:
    condition, valid = line.split()
    valid = tuple(map(int, valid.split(',')))
    result += count(condition, valid)
print(result)

p2 = 0
# Part 2
for line in lines:
    condition, valid = line.split()
    valid = tuple(map(int, valid.split(',')))
    condition = '?'.join([condition]*5)
    valid *= 5
    p2 += count(condition, valid)
print(p2)
