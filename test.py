import rselib

print('Testing Simple Encryption Library version 2')

original = 'Hello ROT13 encryption!'
encrypted = rselib.rot13(original)
decrypted = rselib.rot13(encrypted)

print(f'Original: {original}')
print(f'Encrypted: {encrypted}')
print(f'Decrypted: {decrypted}')
