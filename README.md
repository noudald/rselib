# Rust Simple Encryption Library for Python

This is a simple encryption library, implemented ROT13, created for myself to learn how to create rust extentions for Python.
ROT13 is an incredible stupid, but universally accepted encryption algorithm.
A friend once told me that if you can implement a <insert language here> ROT13 extention for Python, you can implement everything in that specific language for Python.

# Build and Install

Create a new python environment and install the requirements (Maturin)

```
python -m venv .venv
source .venv/bin/activate
pip install -U pip
pip install -r requirements.txt
```

Next compile and deploy rselib into the local Python environment.

```
maturin develop
```

# How to use

In Python

```
>>> import rselib
>>> rselib.rot13('Hello ROT13 encryption!')
'Uryyb EBG13 rapelcgvba!'
>>> rselib.rot13('Uryyb EBG13 rapelcgvba!')
'Hello ROT13 encryption!'
```
