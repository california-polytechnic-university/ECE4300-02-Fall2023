# Import necessary libraries
import socket
from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric import dh
from cryptography.hazmat.backends import default_backend
import time
import psutil

# Function to generate a key pair (private and public key)
def generate_key_pair():
    # Generate Diffie-Hellman parameters
    parameters = dh.generate_parameters(generator=2, key_size=2048, backend=default_backend())
    
    # Generate a private key using the parameters
    private_key = parameters.generate_private_key()
    
    # Derive the corresponding public key
    public_key = private_key.public_key()
    
    return private_key, public_key

# Function to compute the shared key using private and peer public keys
def compute_shared_key(private_key, peer_public_key):
    print("Before key exchange:")
    
    # Display private key in PEM format
    private_key_bytes = private_key.private_bytes(
        encoding=serialization.Encoding.PEM,
        format=serialization.PrivateFormat.PKCS8,
        encryption_algorithm=serialization.NoEncryption()
    )
    print("Private Key (bytes):", private_key_bytes.decode('utf-8'))

    # Display peer public key in PEM format
    peer_public_key_bytes = peer_public_key.public_bytes(
        encoding=serialization.Encoding.PEM,
        format=serialization.PublicFormat.SubjectPublicKeyInfo
    )
    print("Peer Public Key (bytes):", peer_public_key_bytes.decode('utf-8'))

    try:
        # Extract private and public key numbers
        private_key_numbers = private_key.private_numbers()
        peer_public_key_numbers = peer_public_key.public_numbers()

        print("Private Key Numbers:", private_key_numbers)
        print("Peer Public Key Numbers:", peer_public_key_numbers)

        # Reconstruct private and public keys from numbers
        private_key = private_key_numbers.private_key(default_backend())
        peer_public_key = peer_public_key_numbers.public_key(default_backend())

        # Compute the shared key using the reconstructed keys
        shared_key = private_key.exchange(peer_public_key)
        return shared_key
    except Exception as e:
        print(f"Error computing shared key: {e}")
        raise

# Function to send a public key over a socket
def send_key(sock, key):
    # Encode the public key in DER format
    der_encoded_key = key.public_bytes(
        encoding=serialization.Encoding.DER,
        format=serialization.PublicFormat.SubjectPublicKeyInfo
    )
    
    # Send the DER encoded key over the socket
    sock.sendall(der_encoded_key)

# Function to receive a public key from a socket
def receive_key(sock):
    # Receive data from the socket
    data = sock.recv(4096)
    try:
        # Load the DER encoded data as a public key
        return serialization.load_der_public_key(data, backend=default_backend())
    except Exception as e:
        print(f"Error receiving public key: {e}")
        raise

# Set up a socket for communication
listen_address = ('localhost', 15789)
server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
server_socket.bind(listen_address)
server_socket.listen(1)

print(f"Listening on {listen_address}")

# Accept a connection from the client (virtual machine)
client_socket, client_address = server_socket.accept()
print(f"Connection accepted from {client_address}")

# Alice's side
start_time = time.time()
alice_private_key, alice_public_key = generate_key_pair()
end_time = time.time()
print(f"Alice's key generation time: {end_time - start_time} seconds")

# Receive Bob's public key from the virtual machine
try:
    start_time = time.time()
    received_bob_public_key = receive_key(client_socket)
    end_time = time.time()
    print(f"Bob's public key reception time: {end_time - start_time} seconds")
except Exception as e:
    print(f"Error receiving Bob's public key: {e}")
    client_socket.close()
    server_socket.close()
    exit()

# Send Alice's public key to the virtual machine
start_time = time.time()
send_key(client_socket, alice_public_key)
end_time = time.time()
print(f"Alice's public key transmission time: {end_time - start_time} seconds")

# Send Bob's public key to the virtual machine
start_time = time.time()
send_key(client_socket, received_bob_public_key)
end_time = time.time()
print(f"Bob's public key transmission time: {end_time - start_time} seconds")

# Bob's side
start_time = time.time()
bob_private_key, bob_public_key = generate_key_pair()
end_time = time.time()
print(f"Bob's key generation time: {end_time - start_time} seconds")

# Receive Alice's public key from the virtual machine
try:
    start_time = time.time()
    received_alice_public_key = receive_key(client_socket)
    end_time = time.time()
    print(f"Alice's public key reception time: {end_time - start_time} seconds")
except Exception as e:
    print(f"Error receiving Alice's public key: {e}")
    client_socket.close()
    server_socket.close()
    exit()

# Compute shared keys
try:
    start_time = time.time()

    # Compute shared keys
    alice_shared_key = compute_shared_key(alice_private_key, received_bob_public_key)
    bob_shared_key = compute_shared_key(bob_private_key, received_alice_public_key)

    end_time = time.time()
    print("Alice's shared key:", alice_shared_key)
    print("Bob's shared key:", bob_shared_key)
    print(f"Key exchange and computation time: {end_time - start_time} seconds")

except Exception as e:
    print(f"Error computing shared keys: {e}")

# Monitor hardware usage
cpu_percent = psutil.cpu_percent()
memory_percent = psutil.virtual_memory().percent
print(f"CPU Usage: {cpu_percent}%")
print(f"Memory Usage: {memory_percent}%")

# Close the sockets
client_socket.close()
server_socket.close()
