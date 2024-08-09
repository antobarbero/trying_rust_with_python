from intro_py_rs import fibonacci
import time


def run(method, n):
    start = time.time()
    method(n)
    end = time.time()
    print(f"{method.__name__}\tTime taken: {(end-start)*10**3:.03f}ms")


def fibonacci_python(n):
    """pythonic fibonacci function"""
    if n == 0: return 0
    if n == 1: return 1
    return fibonacci_python(n-1) + fibonacci_python(n-2)


if __name__ == '__main__':
    run(fibonacci_python, 40)
    run(fibonacci, 40)
