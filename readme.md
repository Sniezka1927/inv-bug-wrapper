# Prequesites

- Docker

# Run tests

- make all-dockerized
- cargo test -- --show-output

If there's and error while running `make all-dockerized` run

```bash
sudo chmod 666 /var/run/docker.sock
```
